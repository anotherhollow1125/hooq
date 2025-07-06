use crate::impls::option::context::{PartialReplaceContext, ReplaceKind};
use proc_macro2::Span;
use quote::ToTokens;
use syn::spanned::Spanned;
use syn::{
    Attribute, Expr, ExprCall, ExprPath, ImplItem, ImplItemConst, Item, ItemConst, ItemMod,
    ItemStatic, Local, LocalInit, Stmt, TraitItem, TraitItemConst, TraitItemFn, parse_quote,
};

use crate::impls::inert_attr::{InertAttrOption, extract_hooq_info_from_attrs};
pub use crate::impls::option::HooqOption;
use crate::impls::utils::{get_attrs_from_expr, path_is_special_call_like_err};

use super::utils::return_type_is_result;

#[cfg(test)]
mod test;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TailExprTargetKind {
    FnBlockTailExpr,
    BlockTailExpr,
    NotTarget,
}

impl TailExprTargetKind {
    fn is_target(&self) -> bool {
        matches!(
            self,
            TailExprTargetKind::FnBlockTailExpr | TailExprTargetKind::BlockTailExpr
        )
    }
}

fn handle_tail_expr(
    expr: &mut Expr,
    option: &HooqOption,
    context: &PartialReplaceContext,
    tail_expr_target_kind: TailExprTargetKind,
) -> syn::Result<()> {
    let Some(attrs) = get_attrs_from_expr(expr) else {
        return Ok(());
    };
    let HandleInertAttrsResult {
        is_skiped,
        new_context: context,
    } = handle_inert_attrs(attrs, context)?;

    walk_expr(expr, option, &context)?;

    // 念のためここでもターゲットであることを確認
    if !tail_expr_target_kind.is_target() {
        return Ok(());
    }

    // 関数orクロージャで返り値型がResultの時は常にフック
    if context.return_type_is_result()
        && tail_expr_target_kind == TailExprTargetKind::FnBlockTailExpr
    {
        let q_span = expr.span();

        replace_expr(
            !is_skiped,
            expr,
            ReplaceKind::TailExpr,
            q_span,
            option,
            &context,
        )?;

        return Ok(());
    }

    // Ok or Err の時にフック
    if let Expr::Call(ExprCall { func, .. }) = expr
        && let Expr::Path(ExprPath { path, .. }) = *func.clone()
        && path_is_special_call_like_err(&path)
    {
        let q_span = path.span();

        replace_expr(
            !is_skiped,
            expr,
            ReplaceKind::TailExpr,
            q_span,
            option,
            &context,
        )?;

        return Ok(());
    }

    Ok(())
}

pub fn walk_stmt(
    stmt: &mut Stmt,
    tail_expr_target_kind: TailExprTargetKind,
    option: &HooqOption,
    context: &PartialReplaceContext,
) -> syn::Result<()> {
    match stmt {
        Stmt::Local(Local {
            attrs,
            init: Some(LocalInit { expr, diverge, .. }),
            ..
        }) => {
            let HandleInertAttrsResult {
                is_skiped: _,
                new_context: context,
            } = handle_inert_attrs(attrs, context)?;

            walk_expr(expr, option, &context)?;
            if let Some((_, expr_else)) = diverge {
                walk_expr(expr_else, option, &context)?;
            }

            Ok(())
        }
        Stmt::Item(item) => walk_item(item, option, context),

        // 以下は返り値となる Expr
        // 次の場合にフックすることにしたい
        // - 返り値型がResultな関数・クロージャ内部の時: 常にフック
        // - 上記以外: `Ok` | `Err` の時にフック
        Stmt::Expr(expr, None) if tail_expr_target_kind.is_target() => {
            handle_tail_expr(expr, option, context, tail_expr_target_kind)
        }
        Stmt::Expr(expr, _) => walk_expr(expr, option, context),

        // 以下では何もしない
        Stmt::Local(Local { init: None, .. }) | Stmt::Macro(_) => Ok(()),
    }
}

struct HandleInertAttrsResult<'a> {
    is_skiped: bool,
    new_context: PartialReplaceContext<'a>,
}

fn handle_inert_attrs<'a>(
    attrs: &mut Vec<Attribute>,
    context: &'a PartialReplaceContext,
) -> syn::Result<HandleInertAttrsResult<'a>> {
    let InertAttrOption {
        is_skiped,
        is_skiped_all,
        tag,
        method,
    } = extract_hooq_info_from_attrs(attrs)?;

    Ok(HandleInertAttrsResult {
        is_skiped: is_skiped || is_skiped_all || context.is_skiped_all(),
        new_context: PartialReplaceContext::new(context, tag, method, None, is_skiped_all),
    })
}

// 以降、本来的にはVisitやFoldで実装するべきかもしれないが、
// 網羅性の確認のため全て明示的に実装した
// Visitは不変参照を扱うのが前提で、Foldだと書き換えにくそう。
// またもしこれらで行えたとして、結局各要素のAttributeを見ていく必要があるので、記述量に差はないと考えられる。
// ...ともかくTODO

fn walk_item(
    item: &mut Item,
    option: &HooqOption,
    context: &PartialReplaceContext,
) -> syn::Result<()> {
    match item {
        Item::Fn(item_fn) => {
            let HandleInertAttrsResult {
                is_skiped: _,
                new_context: mut context,
            } = handle_inert_attrs(&mut item_fn.attrs, context)?;

            context.update_return_type_is_result(return_type_is_result(&item_fn.sig.output));

            let stmts_len = item_fn.block.stmts.len();
            item_fn
                .block
                .stmts
                .iter_mut()
                .enumerate()
                .map(|(i, stmt)| {
                    let tail_expr_target_kind = if i == stmts_len - 1 {
                        TailExprTargetKind::FnBlockTailExpr
                    } else {
                        TailExprTargetKind::NotTarget
                    };

                    walk_stmt(stmt, tail_expr_target_kind, option, &context)
                })
                .collect::<syn::Result<Vec<()>>>()?;

            Ok(())
        }
        Item::Impl(item_impl) => {
            let HandleInertAttrsResult {
                is_skiped: _,
                new_context: context,
            } = handle_inert_attrs(&mut item_impl.attrs, context)?;

            item_impl
                .items
                .iter_mut()
                .map(|impl_item| {
                    match impl_item {
                        ImplItem::Fn(impl_item_fn) => {
                            let HandleInertAttrsResult {
                                is_skiped: _,
                                new_context: mut context,
                            } = handle_inert_attrs(&mut impl_item_fn.attrs, &context)?;

                            context.update_return_type_is_result(return_type_is_result(
                                &impl_item_fn.sig.output,
                            ));

                            let stmts_len = impl_item_fn.block.stmts.len();
                            impl_item_fn
                                .block
                                .stmts
                                .iter_mut()
                                .enumerate()
                                .map(|(i, stmt)| {
                                    let tail_expr_target_kind = if i == stmts_len - 1 {
                                        TailExprTargetKind::FnBlockTailExpr
                                    } else {
                                        TailExprTargetKind::NotTarget
                                    };

                                    walk_stmt(stmt, tail_expr_target_kind, option, &context)
                                })
                                .collect::<syn::Result<Vec<()>>>()?;

                            Ok(())
                        }

                        ImplItem::Const(ImplItemConst { attrs, expr, .. }) => {
                            let HandleInertAttrsResult {
                                is_skiped: _,
                                new_context: context,
                            } = handle_inert_attrs(attrs, &context)?;

                            walk_expr(expr, option, &context)
                        }

                        // TODO
                        ImplItem::Macro(_) => Ok(()),

                        // 以下の場合何もしない
                        ImplItem::Type(_) | ImplItem::Verbatim(_) | _ => Ok(()),
                    }
                })
                .collect::<syn::Result<Vec<()>>>()?;

            Ok(())
        }
        Item::Mod(ItemMod {
            attrs,
            content: Some((_, items)),
            ..
        }) => {
            let HandleInertAttrsResult {
                is_skiped: _,
                new_context: context,
            } = handle_inert_attrs(attrs, context)?;

            items
                .iter_mut()
                .map(|item| walk_item(item, option, &context))
                .collect::<syn::Result<Vec<()>>>()?;

            Ok(())
        }

        Item::Const(ItemConst { attrs, expr, .. })
        | Item::Static(ItemStatic { attrs, expr, .. }) => {
            let HandleInertAttrsResult {
                is_skiped: _,
                new_context: context,
            } = handle_inert_attrs(attrs, context)?;

            walk_expr(expr, option, &context)
        }

        Item::Trait(item_trait) => {
            let HandleInertAttrsResult {
                is_skiped: _,
                new_context: context,
            } = handle_inert_attrs(&mut item_trait.attrs, context)?;

            item_trait
                .items
                .iter_mut()
                .map(|trait_item| {
                    match trait_item {
                        TraitItem::Const(TraitItemConst {
                            attrs,
                            default: Some((_, expr)),
                            ..
                        }) => {
                            let HandleInertAttrsResult {
                                is_skiped: _,
                                new_context: context,
                            } = handle_inert_attrs(attrs, &context)?;

                            walk_expr(expr, option, &context)
                        }
                        TraitItem::Fn(TraitItemFn {
                            attrs,
                            sig,
                            default: Some(block),
                            ..
                        }) => {
                            let HandleInertAttrsResult {
                                is_skiped: _,
                                new_context: mut context,
                            } = handle_inert_attrs(attrs, &context)?;

                            context
                                .update_return_type_is_result(return_type_is_result(&sig.output));

                            let stmts_len = block.stmts.len();
                            block
                                .stmts
                                .iter_mut()
                                .enumerate()
                                .map(|(i, stmt)| {
                                    let tail_expr_target_kind = if i == stmts_len - 1 {
                                        TailExprTargetKind::FnBlockTailExpr
                                    } else {
                                        TailExprTargetKind::NotTarget
                                    };

                                    walk_stmt(stmt, tail_expr_target_kind, option, &context)
                                })
                                .collect::<syn::Result<Vec<()>>>()?;

                            Ok(())
                        }

                        // TODO
                        TraitItem::Macro(_) => Ok(()),

                        // 以下の場合何もしない
                        TraitItem::Const(TraitItemConst { default: None, .. })
                        | TraitItem::Fn(TraitItemFn { default: None, .. })
                        | TraitItem::Type(_)
                        | TraitItem::Verbatim(_)
                        | _ => Ok(()),
                    }
                })
                .collect::<syn::Result<Vec<()>>>()?;

            Ok(())
        }
        // TODO
        Item::Macro(_) => Ok(()),

        // 以下では何もしない
        // Item::TraitAlias は将来のために予約された要素
        Item::Enum(_)
        | Item::ExternCrate(_)
        | Item::ForeignMod(_)
        | Item::Mod(ItemMod { content: None, .. })
        | Item::Struct(_)
        | Item::TraitAlias(_)
        | Item::Type(_)
        | Item::Union(_)
        | Item::Use(_)
        | Item::Verbatim(_)
        | _ => Ok(()),
    }
}

fn replace_expr(
    apply: bool,
    expr_field: &mut Expr,
    kind: ReplaceKind,
    q_span: Span,
    option: &HooqOption,
    context: &PartialReplaceContext,
) -> syn::Result<()> {
    if !apply {
        return Ok(());
    }

    context.counter.borrow_mut().count_up(kind);
    let context = context.as_replace_context(expr_field.to_token_stream().to_string(), kind);

    let method = option.generate_method(q_span, &context)?;
    let original_expr = expr_field.clone();

    *expr_field = parse_quote! {
        #original_expr #method
    };

    Ok(())
}

fn walk_expr(
    expr: &mut Expr,
    option: &HooqOption,
    context: &PartialReplaceContext,
) -> syn::Result<()> {
    match expr {
        // 置換対象となるバリアント
        Expr::Try(expr_try) => {
            let HandleInertAttrsResult {
                is_skiped,
                new_context: context,
            } = handle_inert_attrs(&mut expr_try.attrs, context)?;

            walk_expr(&mut expr_try.expr, option, &context)?;

            let q_span = expr_try.question_token.span();

            replace_expr(
                !is_skiped,
                &mut expr_try.expr,
                ReplaceKind::Question,
                q_span,
                option,
                &context,
            )?;

            Ok(())
        }
        Expr::Return(expr_return) => {
            let HandleInertAttrsResult {
                is_skiped,
                new_context: context,
            } = handle_inert_attrs(&mut expr_return.attrs, context)?;

            if let Some(expr) = expr_return.expr.as_mut() {
                walk_expr(expr, option, &context)?;

                let q_span = expr_return.return_token.span();

                let is_ok_or_err = if let Expr::Call(ExprCall { func, .. }) = &**expr
                    && let Expr::Path(ExprPath { path, .. }) = *func.clone()
                    && path_is_special_call_like_err(&path)
                {
                    true
                } else {
                    false
                };

                // 返り値型がResultの時 or OkやErrの時フック
                if is_ok_or_err || context.return_type_is_result() {
                    replace_expr(
                        !is_skiped,
                        expr,
                        ReplaceKind::Return,
                        q_span,
                        option,
                        &context,
                    )?;
                }
            }

            Ok(())
        }

        // ネストの中身を見る必要があるバリアント
        Expr::Array(expr_array) => {
            let HandleInertAttrsResult {
                is_skiped: _,
                new_context: context,
            } = handle_inert_attrs(&mut expr_array.attrs, context)?;

            expr_array
                .elems
                .iter_mut()
                .map(|expr| walk_expr(expr, option, &context))
                .collect::<syn::Result<Vec<()>>>()?;

            Ok(())
        }
        Expr::Assign(expr_assign) => {
            let HandleInertAttrsResult {
                is_skiped: _,
                new_context: context,
            } = handle_inert_attrs(&mut expr_assign.attrs, context)?;

            walk_expr(&mut expr_assign.right, option, &context)
        }
        Expr::Async(expr_async) => {
            let HandleInertAttrsResult {
                is_skiped: _,
                new_context: context,
            } = handle_inert_attrs(&mut expr_async.attrs, context)?;

            let stmts_len = expr_async.block.stmts.len();

            expr_async
                .block
                .stmts
                .iter_mut()
                .enumerate()
                .map(|(i, stmt)| {
                    let tail_expr_target_kind = if i == stmts_len - 1 {
                        TailExprTargetKind::BlockTailExpr
                    } else {
                        TailExprTargetKind::NotTarget
                    };

                    walk_stmt(stmt, tail_expr_target_kind, option, &context)
                })
                .collect::<syn::Result<Vec<()>>>()?;

            Ok(())
        }
        Expr::Await(expr_await) => {
            let HandleInertAttrsResult {
                is_skiped: _,
                new_context: context,
            } = handle_inert_attrs(&mut expr_await.attrs, context)?;

            walk_expr(&mut expr_await.base, option, &context)
        }
        Expr::Binary(expr_binary) => {
            let HandleInertAttrsResult {
                is_skiped: _,
                new_context: context,
            } = handle_inert_attrs(&mut expr_binary.attrs, context)?;

            walk_expr(&mut expr_binary.left, option, &context)?;
            walk_expr(&mut expr_binary.right, option, &context)
        }
        Expr::Block(expr_block) => {
            let HandleInertAttrsResult {
                is_skiped: _,
                new_context: context,
            } = handle_inert_attrs(&mut expr_block.attrs, context)?;

            let stmts_len = expr_block.block.stmts.len();

            expr_block
                .block
                .stmts
                .iter_mut()
                .enumerate()
                .map(|(i, stmt)| {
                    let tail_expr_target_kind = if i == stmts_len - 1 {
                        TailExprTargetKind::BlockTailExpr
                    } else {
                        TailExprTargetKind::NotTarget
                    };

                    walk_stmt(stmt, tail_expr_target_kind, option, &context)
                })
                .collect::<syn::Result<Vec<()>>>()?;

            Ok(())
        }
        Expr::Break(expr_break) => {
            let HandleInertAttrsResult {
                is_skiped: _,
                new_context: context,
            } = handle_inert_attrs(&mut expr_break.attrs, context)?;

            if let Some(res) = expr_break.expr.as_mut() {
                walk_expr(res, option, &context)?;
            }

            Ok(())
        }
        Expr::Call(expr_call) => {
            let HandleInertAttrsResult {
                is_skiped: _,
                new_context: context,
            } = handle_inert_attrs(&mut expr_call.attrs, context)?;

            walk_expr(&mut expr_call.func, option, &context)?;
            expr_call
                .args
                .iter_mut()
                .map(|expr| walk_expr(expr, option, &context))
                .collect::<syn::Result<Vec<()>>>()?;

            Ok(())
        }
        Expr::Cast(expr_cast) => {
            let HandleInertAttrsResult {
                is_skiped: _,
                new_context: context,
            } = handle_inert_attrs(&mut expr_cast.attrs, context)?;

            walk_expr(&mut expr_cast.expr, option, &context)
        }
        Expr::Closure(expr_closure) => {
            let HandleInertAttrsResult {
                is_skiped: _,
                new_context: mut context,
            } = handle_inert_attrs(&mut expr_closure.attrs, context)?;

            context.update_return_type_is_result(return_type_is_result(&expr_closure.output));

            match &mut *expr_closure.body {
                Expr::Block(expr_block) => {
                    let HandleInertAttrsResult {
                        is_skiped: _,
                        new_context: context,
                    } = handle_inert_attrs(&mut expr_block.attrs, &context)?;

                    let stmts_len = expr_block.block.stmts.len();
                    expr_block
                        .block
                        .stmts
                        .iter_mut()
                        .enumerate()
                        .map(|(i, stmt)| {
                            let tail_expr_target_kind = if i == stmts_len - 1 {
                                TailExprTargetKind::FnBlockTailExpr
                            } else {
                                TailExprTargetKind::NotTarget
                            };

                            walk_stmt(stmt, tail_expr_target_kind, option, &context)
                        })
                        .collect::<syn::Result<Vec<()>>>()?;
                }
                e => handle_tail_expr(e, option, &context, TailExprTargetKind::FnBlockTailExpr)?,
            }

            Ok(())
        }
        Expr::Const(expr_const) => {
            let HandleInertAttrsResult {
                is_skiped: _,
                new_context: context,
            } = handle_inert_attrs(&mut expr_const.attrs, context)?;

            let stmts_len = expr_const.block.stmts.len();

            expr_const
                .block
                .stmts
                .iter_mut()
                .enumerate()
                .map(|(i, stmt)| {
                    let tail_expr_target_kind = if i == stmts_len - 1 {
                        TailExprTargetKind::BlockTailExpr
                    } else {
                        TailExprTargetKind::NotTarget
                    };

                    walk_stmt(stmt, tail_expr_target_kind, option, &context)
                })
                .collect::<syn::Result<Vec<()>>>()?;

            Ok(())
        }
        Expr::Field(expr_field) => {
            let HandleInertAttrsResult {
                is_skiped: _,
                new_context: context,
            } = handle_inert_attrs(&mut expr_field.attrs, context)?;

            walk_expr(&mut expr_field.base, option, &context)
        }
        Expr::ForLoop(expr_for_loop) => {
            let HandleInertAttrsResult {
                is_skiped: _,
                new_context: context,
            } = handle_inert_attrs(&mut expr_for_loop.attrs, context)?;

            walk_expr(&mut expr_for_loop.expr, option, &context)?;

            let stmts_len = expr_for_loop.body.stmts.len();

            expr_for_loop
                .body
                .stmts
                .iter_mut()
                .enumerate()
                .map(|(i, stmt)| {
                    let tail_expr_target_kind = if i == stmts_len - 1 {
                        TailExprTargetKind::BlockTailExpr
                    } else {
                        TailExprTargetKind::NotTarget
                    };

                    walk_stmt(stmt, tail_expr_target_kind, option, &context)
                })
                .collect::<syn::Result<Vec<()>>>()?;

            Ok(())
        }
        Expr::Group(expr_group) => {
            let HandleInertAttrsResult {
                is_skiped: _,
                new_context: context,
            } = handle_inert_attrs(&mut expr_group.attrs, context)?;

            walk_expr(&mut expr_group.expr, option, &context)
        }
        Expr::If(expr_if) => {
            let HandleInertAttrsResult {
                is_skiped: _,
                new_context: context,
            } = handle_inert_attrs(&mut expr_if.attrs, context)?;

            walk_expr(&mut expr_if.cond, option, &context)?;

            let stmts_len = expr_if.then_branch.stmts.len();

            expr_if
                .then_branch
                .stmts
                .iter_mut()
                .enumerate()
                .map(|(i, stmt)| {
                    let tail_expr_target_kind = if i == stmts_len - 1 {
                        TailExprTargetKind::BlockTailExpr
                    } else {
                        TailExprTargetKind::NotTarget
                    };

                    walk_stmt(stmt, tail_expr_target_kind, option, &context)
                })
                .collect::<syn::Result<Vec<()>>>()?;

            if let Some((_, else_branch)) = expr_if.else_branch.as_mut() {
                walk_expr(else_branch, option, &context)?;
            }

            Ok(())
        }
        Expr::Index(expr_index) => {
            let HandleInertAttrsResult {
                is_skiped: _,
                new_context: context,
            } = handle_inert_attrs(&mut expr_index.attrs, context)?;

            walk_expr(&mut expr_index.expr, option, &context)?;
            walk_expr(&mut expr_index.index, option, &context)
        }
        Expr::Let(expr_let) => {
            let HandleInertAttrsResult {
                is_skiped: _,
                new_context: context,
            } = handle_inert_attrs(&mut expr_let.attrs, context)?;

            walk_expr(&mut expr_let.expr, option, &context)
        }
        Expr::Loop(expr_loop) => {
            let HandleInertAttrsResult {
                is_skiped: _,
                new_context: context,
            } = handle_inert_attrs(&mut expr_loop.attrs, context)?;

            let stmts_len = expr_loop.body.stmts.len();

            expr_loop
                .body
                .stmts
                .iter_mut()
                .enumerate()
                .map(|(i, stmt)| {
                    let tail_expr_target_kind = if i == stmts_len - 1 {
                        TailExprTargetKind::BlockTailExpr
                    } else {
                        TailExprTargetKind::NotTarget
                    };

                    walk_stmt(stmt, tail_expr_target_kind, option, &context)
                })
                .collect::<syn::Result<Vec<()>>>()?;

            Ok(())
        }
        Expr::Match(expr_match) => {
            let HandleInertAttrsResult {
                is_skiped: _,
                new_context: context,
            } = handle_inert_attrs(&mut expr_match.attrs, context)?;

            walk_expr(&mut expr_match.expr, option, &context)?;
            expr_match
                .arms
                .iter_mut()
                .map(|arm| {
                    if let Some((_, guard)) = arm.guard.as_mut() {
                        walk_expr(guard, option, &context)?;
                    }
                    walk_expr(&mut arm.body, option, &context)?;

                    Ok(())
                })
                .collect::<syn::Result<Vec<()>>>()?;

            Ok(())
        }
        Expr::MethodCall(expr_method_call) => {
            let HandleInertAttrsResult {
                is_skiped: _,
                new_context: context,
            } = handle_inert_attrs(&mut expr_method_call.attrs, context)?;

            walk_expr(&mut expr_method_call.receiver, option, &context)?;
            expr_method_call
                .args
                .iter_mut()
                .map(|expr| walk_expr(expr, option, &context))
                .collect::<syn::Result<Vec<()>>>()?;

            Ok(())
        }
        Expr::Paren(expr_paren) => {
            let HandleInertAttrsResult {
                is_skiped: _,
                new_context: context,
            } = handle_inert_attrs(&mut expr_paren.attrs, context)?;

            walk_expr(&mut expr_paren.expr, option, &context)
        }
        Expr::Range(expr_range) => {
            let HandleInertAttrsResult {
                is_skiped: _,
                new_context: context,
            } = handle_inert_attrs(&mut expr_range.attrs, context)?;

            if let Some(expr_start) = expr_range.start.as_mut() {
                walk_expr(expr_start, option, &context)?;
            }
            if let Some(expr_end) = expr_range.end.as_mut() {
                walk_expr(expr_end, option, &context)?;
            }

            Ok(())
        }
        Expr::RawAddr(expr_raw_addr) => {
            let HandleInertAttrsResult {
                is_skiped: _,
                new_context: context,
            } = handle_inert_attrs(&mut expr_raw_addr.attrs, context)?;

            walk_expr(&mut expr_raw_addr.expr, option, &context)
        }
        Expr::Reference(expr_reference) => {
            let HandleInertAttrsResult {
                is_skiped: _,
                new_context: context,
            } = handle_inert_attrs(&mut expr_reference.attrs, context)?;

            walk_expr(&mut expr_reference.expr, option, &context)
        }
        Expr::Repeat(expr_repeat) => {
            let HandleInertAttrsResult {
                is_skiped: _,
                new_context: context,
            } = handle_inert_attrs(&mut expr_repeat.attrs, context)?;

            walk_expr(&mut expr_repeat.expr, option, &context)?;
            walk_expr(&mut expr_repeat.len, option, &context)
        }
        Expr::Struct(expr_struct) => {
            let HandleInertAttrsResult {
                is_skiped: _,
                new_context: context,
            } = handle_inert_attrs(&mut expr_struct.attrs, context)?;

            expr_struct
                .fields
                .iter_mut()
                .map(|field| {
                    let HandleInertAttrsResult {
                        is_skiped: _,
                        new_context: context,
                    } = handle_inert_attrs(&mut field.attrs, &context)?;

                    walk_expr(&mut field.expr, option, &context)
                })
                .collect::<syn::Result<Vec<()>>>()?;
            if let Some(rest) = expr_struct.rest.as_mut() {
                walk_expr(rest, option, &context)?;
            }

            Ok(())
        }
        Expr::TryBlock(expr_try_block) => {
            let HandleInertAttrsResult {
                is_skiped: _,
                new_context: context,
            } = handle_inert_attrs(&mut expr_try_block.attrs, context)?;

            let stmts_len = expr_try_block.block.stmts.len();

            // nightly な機能と考えられるためテストしない

            expr_try_block
                .block
                .stmts
                .iter_mut()
                .enumerate()
                .map(|(i, stmt)| {
                    let tail_expr_target_kind = if i == stmts_len - 1 {
                        TailExprTargetKind::BlockTailExpr
                    } else {
                        TailExprTargetKind::NotTarget
                    };

                    walk_stmt(stmt, tail_expr_target_kind, option, &context)
                })
                .collect::<syn::Result<Vec<()>>>()?;

            Ok(())
        }
        Expr::Tuple(expr_tuple) => {
            let HandleInertAttrsResult {
                is_skiped: _,
                new_context: context,
            } = handle_inert_attrs(&mut expr_tuple.attrs, context)?;

            expr_tuple
                .elems
                .iter_mut()
                .map(|expr| walk_expr(expr, option, &context))
                .collect::<syn::Result<Vec<()>>>()?;

            Ok(())
        }
        Expr::Unary(expr_unary) => {
            let HandleInertAttrsResult {
                is_skiped: _,
                new_context: context,
            } = handle_inert_attrs(&mut expr_unary.attrs, context)?;

            walk_expr(&mut expr_unary.expr, option, &context)
        }
        Expr::Unsafe(expr_unsafe) => {
            let HandleInertAttrsResult {
                is_skiped: _,
                new_context: context,
            } = handle_inert_attrs(&mut expr_unsafe.attrs, context)?;

            let stmts_len = expr_unsafe.block.stmts.len();

            expr_unsafe
                .block
                .stmts
                .iter_mut()
                .enumerate()
                .map(|(i, stmt)| {
                    let tail_expr_target_kind = if i == stmts_len - 1 {
                        TailExprTargetKind::BlockTailExpr
                    } else {
                        TailExprTargetKind::NotTarget
                    };

                    walk_stmt(stmt, tail_expr_target_kind, option, &context)
                })
                .collect::<syn::Result<Vec<()>>>()?;

            Ok(())
        }
        Expr::While(expr_while) => {
            let HandleInertAttrsResult {
                is_skiped: _,
                new_context: context,
            } = handle_inert_attrs(&mut expr_while.attrs, context)?;

            walk_expr(&mut expr_while.cond, option, &context)?;

            let stmts_len = expr_while.body.stmts.len();

            expr_while
                .body
                .stmts
                .iter_mut()
                .enumerate()
                .map(|(i, stmt)| {
                    let tail_expr_target_kind = if i == stmts_len - 1 {
                        TailExprTargetKind::BlockTailExpr
                    } else {
                        TailExprTargetKind::NotTarget
                    };

                    walk_stmt(stmt, tail_expr_target_kind, option, &context)
                })
                .collect::<syn::Result<Vec<()>>>()?;

            Ok(())
        }
        Expr::Yield(expr_yield) => {
            let HandleInertAttrsResult {
                is_skiped: _,
                new_context: context,
            } = handle_inert_attrs(&mut expr_yield.attrs, context)?;

            // nightly な機能と考えられるためテストしない

            if let Some(expr) = expr_yield.expr.as_mut() {
                walk_expr(expr, option, &context)?;
            }

            Ok(())
        }

        // 以下では何もしない
        // 欲を言えばExpr::Macro(_)の中も見たいが、独自文法となってしまっているマクロもあるので諦める
        Expr::Continue(_)
        | Expr::Infer(_)
        | Expr::Lit(_)
        | Expr::Macro(_)
        | Expr::Path(_)
        | Expr::Verbatim(_)
        | _ => Ok(()),
    }
}
