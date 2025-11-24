use quote::ToTokens;
use syn::spanned::Spanned;
use syn::{
    Expr, ExprCall, ExprMacro, ExprPath, ImplItem, ImplItemConst, ImplItemMacro, Item, ItemConst,
    ItemMacro, ItemMod, ItemStatic, Local, LocalInit, Stmt, StmtMacro, TraitItem, TraitItemConst,
    TraitItemFn, TraitItemMacro,
};

use crate::impls::inert_attr::context::{HookContext, HookTargetKind};
use crate::impls::inert_attr::{HandleInertAttrsResult, handle_inert_attrs};
use crate::impls::utils::function_info::{ClosureInfo, FunctionInfo};
use crate::impls::utils::get_attrs_from_expr;

mod hook_expr;
mod walk_macro;

use hook_expr::hook_expr;
use walk_macro::walk_macro;

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

struct TargetCallCheckBools {
    is_target_call: bool,
    is_not_target_call: bool,
}

fn target_call_check_bools(context: &HookContext, expr: &Expr) -> TargetCallCheckBools {
    if let Expr::Call(ExprCall { func, .. }) = expr
        && let Expr::Path(ExprPath { path, .. }) = func.as_ref()
    {
        TargetCallCheckBools {
            is_target_call: context.path_is_hook_call_like_err(path),
            is_not_target_call: context.path_is_not_hook_call_like_ok(path),
        }
    } else {
        TargetCallCheckBools {
            is_target_call: false,
            is_not_target_call: false,
        }
    }
}

fn handle_tail_expr(
    expr: &mut Expr,
    context: &HookContext,
    tail_expr_target_kind: TailExprTargetKind,
) -> syn::Result<()> {
    let Some(attrs) = get_attrs_from_expr(expr) else {
        return Ok(());
    };
    let HandleInertAttrsResult {
        is_skipped,
        new_context: context,
    } = handle_inert_attrs(attrs, context)?;

    let source_tokenstream = expr.to_token_stream();

    walk_expr(expr, &context)?;

    // 念のためここでもターゲットであることを確認
    if !tail_expr_target_kind.is_target() {
        return Ok(());
    }

    let TargetCallCheckBools {
        is_target_call,
        is_not_target_call,
    } = target_call_check_bools(&context, expr);

    // 以下のような場合にフック
    // - Err の時
    // - 関数orクロージャで返り値型がResultでOkではない時
    if is_target_call
        || context.return_type_is_result()
            && tail_expr_target_kind == TailExprTargetKind::FnBlockTailExpr
            && !is_not_target_call
    {
        let q_span = expr.span();

        let _exc = hook_expr(
            !is_skipped,
            expr,
            source_tokenstream,
            HookTargetKind::TailExpr,
            q_span,
            &context,
        )?;

        return Ok(());
    }

    Ok(())
}

fn walk_stmt(
    stmt: &mut Stmt,
    tail_expr_target_kind: TailExprTargetKind,
    context: &HookContext,
) -> syn::Result<()> {
    match stmt {
        Stmt::Local(Local {
            attrs,
            init: Some(LocalInit { expr, diverge, .. }),
            ..
        }) => {
            let HandleInertAttrsResult {
                is_skipped: _,
                new_context: context,
            } = handle_inert_attrs(attrs, context)?;

            walk_expr(expr, &context)?;
            if let Some((_, expr_else)) = diverge {
                walk_expr(expr_else, &context)?;
            }

            Ok(())
        }
        Stmt::Item(item) => walk_item(item, context),

        // 以下は返り値となる Expr
        // 次の場合にフックすることにしたい
        // - 返り値型がResultな関数・クロージャ内部の時: 常にフック
        // - 上記以外: `Ok` | `Err` の時にフック
        Stmt::Expr(expr, None) if tail_expr_target_kind.is_target() => {
            handle_tail_expr(expr, context, tail_expr_target_kind)
        }
        Stmt::Expr(expr, _) => walk_expr(expr, context),

        Stmt::Macro(StmtMacro {
            attrs,
            mac,
            semi_token: _,
        }) => walk_macro(attrs, mac, context),

        // 以下では何もしない
        Stmt::Local(Local { init: None, .. }) => Ok(()),
    }
}

// 以降、本来的にはVisitやFoldで実装するべきかもしれないが、
// 網羅性の確認のため全て明示的に実装した
// Visitは不変参照を扱うのが前提で、Foldだと書き換えにくそう。
// またもしこれらで行えたとして、結局各要素のAttributeを見ていく必要があるので、記述量に差はないと考えられる。
// ...ともかくTODO

pub fn walk_item(item: &mut Item, context: &HookContext) -> syn::Result<()> {
    match item {
        Item::Fn(item_fn) => {
            let HandleInertAttrsResult {
                is_skipped: _,
                new_context: mut context,
            } = handle_inert_attrs(&mut item_fn.attrs, context)?;

            let function_info = FunctionInfo::Function(item_fn.sig.clone());
            context.update_fn_info(function_info);

            let stmts_len = item_fn.block.stmts.len();
            let context = context.for_sub_scope_context();

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

                    walk_stmt(stmt, tail_expr_target_kind, &context)
                })
                .collect::<syn::Result<Vec<()>>>()?;

            Ok(())
        }
        Item::Impl(item_impl) => {
            let HandleInertAttrsResult {
                is_skipped: _,
                new_context: context,
            } = handle_inert_attrs(&mut item_impl.attrs, context)?;

            item_impl
                .items
                .iter_mut()
                .map(|impl_item| {
                    match impl_item {
                        ImplItem::Fn(impl_item_fn) => {
                            let HandleInertAttrsResult {
                                is_skipped: _,
                                new_context: mut context,
                            } = handle_inert_attrs(&mut impl_item_fn.attrs, &context)?;

                            let function_info = FunctionInfo::Function(impl_item_fn.sig.clone());
                            context.update_fn_info(function_info);

                            let stmts_len = impl_item_fn.block.stmts.len();
                            let context = context.for_sub_scope_context();

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

                                    walk_stmt(stmt, tail_expr_target_kind, &context)
                                })
                                .collect::<syn::Result<Vec<()>>>()?;

                            Ok(())
                        }

                        ImplItem::Const(ImplItemConst { attrs, expr, .. }) => {
                            let HandleInertAttrsResult {
                                is_skipped: _,
                                new_context: context,
                            } = handle_inert_attrs(attrs, &context)?;

                            walk_expr(expr, &context)
                        }

                        ImplItem::Macro(ImplItemMacro {
                            attrs,
                            mac,
                            semi_token: _,
                        }) => walk_macro(attrs, mac, &context),

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
                is_skipped: _,
                new_context: context,
            } = handle_inert_attrs(attrs, context)?;

            items
                .iter_mut()
                .map(|item| walk_item(item, &context))
                .collect::<syn::Result<Vec<()>>>()?;

            Ok(())
        }

        Item::Const(ItemConst { attrs, expr, .. })
        | Item::Static(ItemStatic { attrs, expr, .. }) => {
            let HandleInertAttrsResult {
                is_skipped: _,
                new_context: context,
            } = handle_inert_attrs(attrs, context)?;

            walk_expr(expr, &context)
        }

        Item::Trait(item_trait) => {
            let HandleInertAttrsResult {
                is_skipped: _,
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
                                is_skipped: _,
                                new_context: context,
                            } = handle_inert_attrs(attrs, &context)?;

                            walk_expr(expr, &context)
                        }
                        TraitItem::Fn(TraitItemFn {
                            attrs,
                            sig,
                            default: Some(block),
                            ..
                        }) => {
                            let HandleInertAttrsResult {
                                is_skipped: _,
                                new_context: mut context,
                            } = handle_inert_attrs(attrs, &context)?;

                            context.update_fn_info(FunctionInfo::Function(sig.clone()));

                            let stmts_len = block.stmts.len();
                            let context = context.for_sub_scope_context();

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

                                    walk_stmt(stmt, tail_expr_target_kind, &context)
                                })
                                .collect::<syn::Result<Vec<()>>>()?;

                            Ok(())
                        }

                        TraitItem::Macro(TraitItemMacro {
                            attrs,
                            mac,
                            semi_token: _,
                        }) => walk_macro(attrs, mac, &context),

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

        Item::Macro(ItemMacro {
            attrs,
            ident,
            mac,
            semi_token: _,
        }) => {
            if ident.is_some() {
                // ident がSomeということは
                // macro_rules! の定義である
                // ここにフックする必要はない
                return Ok(());
            }

            walk_macro(attrs, mac, context)
        }

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

fn walk_expr(expr: &mut Expr, context: &HookContext) -> syn::Result<()> {
    match expr {
        // 置換対象となるバリアント
        expr @ Expr::Try(..) => {
            // 上記マッチングでマッチすることは検証済みのため、
            // 以下のif式は必ず実行される
            // 目的は可変参照の関係でスコープを分けるため
            let new_expr: Option<Expr> = if let Expr::Try(expr_try) = expr {
                let HandleInertAttrsResult {
                    is_skipped,
                    new_context: context,
                } = handle_inert_attrs(&mut expr_try.attrs, context)?;

                let source_tokenstream = expr_try.to_token_stream();

                walk_expr(&mut expr_try.expr, &context)?;

                let q_span = expr_try.question_token.span();

                let exc = hook_expr(
                    !is_skipped,
                    &mut expr_try.expr,
                    source_tokenstream,
                    HookTargetKind::Question,
                    q_span,
                    &context,
                )?;

                match exc {
                    // ? を消去
                    Some(_) => Some(*expr_try.expr.clone()),
                    // ? を残す
                    None => None,
                }
            } else {
                unreachable!()
            };

            if let Some(new_expr) = new_expr {
                *expr = new_expr;
            }

            Ok(())
        }
        Expr::Return(expr_return) => {
            let HandleInertAttrsResult {
                is_skipped,
                new_context: context,
            } = handle_inert_attrs(&mut expr_return.attrs, context)?;

            let source_tokenstream = expr_return.to_token_stream();

            if let Some(expr) = expr_return.expr.as_mut() {
                walk_expr(expr, &context)?;

                let q_span = expr_return.return_token.span();

                let TargetCallCheckBools {
                    is_target_call,
                    is_not_target_call,
                } = target_call_check_bools(&context, expr);

                // Errの時 or 返り値型がResultでOkでない時フック
                if is_target_call || context.return_type_is_result() && !is_not_target_call {
                    let _exc = hook_expr(
                        !is_skipped,
                        expr,
                        source_tokenstream,
                        HookTargetKind::Return,
                        q_span,
                        &context,
                    )?;
                }
            }

            Ok(())
        }

        // ネストの中身を見る必要があるバリアント
        Expr::Array(expr_array) => {
            let HandleInertAttrsResult {
                is_skipped: _,
                new_context: context,
            } = handle_inert_attrs(&mut expr_array.attrs, context)?;

            expr_array
                .elems
                .iter_mut()
                .map(|expr| walk_expr(expr, &context))
                .collect::<syn::Result<Vec<()>>>()?;

            Ok(())
        }
        Expr::Assign(expr_assign) => {
            let HandleInertAttrsResult {
                is_skipped: _,
                new_context: context,
            } = handle_inert_attrs(&mut expr_assign.attrs, context)?;

            walk_expr(&mut expr_assign.right, &context)
        }
        Expr::Async(expr_async) => {
            let HandleInertAttrsResult {
                is_skipped: _,
                new_context: context,
            } = handle_inert_attrs(&mut expr_async.attrs, context)?;

            let stmts_len = expr_async.block.stmts.len();
            let context = context.for_sub_scope_context();

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

                    walk_stmt(stmt, tail_expr_target_kind, &context)
                })
                .collect::<syn::Result<Vec<()>>>()?;

            Ok(())
        }
        Expr::Await(expr_await) => {
            let HandleInertAttrsResult {
                is_skipped: _,
                new_context: context,
            } = handle_inert_attrs(&mut expr_await.attrs, context)?;

            walk_expr(&mut expr_await.base, &context)
        }
        Expr::Binary(expr_binary) => {
            let HandleInertAttrsResult {
                is_skipped: _,
                new_context: context,
            } = handle_inert_attrs(&mut expr_binary.attrs, context)?;

            walk_expr(&mut expr_binary.left, &context)?;
            walk_expr(&mut expr_binary.right, &context)
        }
        Expr::Block(expr_block) => {
            let HandleInertAttrsResult {
                is_skipped: _,
                new_context: context,
            } = handle_inert_attrs(&mut expr_block.attrs, context)?;

            let stmts_len = expr_block.block.stmts.len();
            let context = context.for_sub_scope_context();

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

                    walk_stmt(stmt, tail_expr_target_kind, &context)
                })
                .collect::<syn::Result<Vec<()>>>()?;

            Ok(())
        }
        Expr::Break(expr_break) => {
            let HandleInertAttrsResult {
                is_skipped: _,
                new_context: context,
            } = handle_inert_attrs(&mut expr_break.attrs, context)?;

            if let Some(res) = expr_break.expr.as_mut() {
                walk_expr(res, &context)?;
            }

            Ok(())
        }
        Expr::Call(expr_call) => {
            let HandleInertAttrsResult {
                is_skipped: _,
                new_context: context,
            } = handle_inert_attrs(&mut expr_call.attrs, context)?;

            walk_expr(&mut expr_call.func, &context)?;
            expr_call
                .args
                .iter_mut()
                .map(|expr| walk_expr(expr, &context))
                .collect::<syn::Result<Vec<()>>>()?;

            Ok(())
        }
        Expr::Cast(expr_cast) => {
            let HandleInertAttrsResult {
                is_skipped: _,
                new_context: context,
            } = handle_inert_attrs(&mut expr_cast.attrs, context)?;

            walk_expr(&mut expr_cast.expr, &context)
        }
        Expr::Closure(expr_closure) => {
            let HandleInertAttrsResult {
                is_skipped: _,
                new_context: mut context,
            } = handle_inert_attrs(&mut expr_closure.attrs, context)?;

            let closure_info =
                ClosureInfo::new(expr_closure.clone(), context.local_context.fn_info.clone());
            context.update_fn_info(closure_info.into());

            match &mut *expr_closure.body {
                Expr::Block(expr_block) => {
                    let HandleInertAttrsResult {
                        is_skipped: _,
                        new_context: context,
                    } = handle_inert_attrs(&mut expr_block.attrs, &context)?;

                    let stmts_len = expr_block.block.stmts.len();
                    let context = context.for_sub_scope_context();

                    expr_block
                        .block
                        .stmts
                        .iter_mut()
                        .enumerate()
                        .map(|(i, stmt)| {
                            let tail_expr_target_kind = if i == stmts_len - 1 {
                                // ここがBlockとは異なる点に注意
                                TailExprTargetKind::FnBlockTailExpr
                            } else {
                                TailExprTargetKind::NotTarget
                            };

                            walk_stmt(stmt, tail_expr_target_kind, &context)
                        })
                        .collect::<syn::Result<Vec<()>>>()?;
                }
                e => handle_tail_expr(e, &context, TailExprTargetKind::FnBlockTailExpr)?,
            }

            Ok(())
        }
        Expr::Const(expr_const) => {
            let HandleInertAttrsResult {
                is_skipped: _,
                new_context: context,
            } = handle_inert_attrs(&mut expr_const.attrs, context)?;

            let stmts_len = expr_const.block.stmts.len();
            let context = context.for_sub_scope_context();

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

                    walk_stmt(stmt, tail_expr_target_kind, &context)
                })
                .collect::<syn::Result<Vec<()>>>()?;

            Ok(())
        }
        Expr::Field(expr_field) => {
            let HandleInertAttrsResult {
                is_skipped: _,
                new_context: context,
            } = handle_inert_attrs(&mut expr_field.attrs, context)?;

            walk_expr(&mut expr_field.base, &context)
        }
        Expr::ForLoop(expr_for_loop) => {
            let HandleInertAttrsResult {
                is_skipped: _,
                new_context: context,
            } = handle_inert_attrs(&mut expr_for_loop.attrs, context)?;

            walk_expr(&mut expr_for_loop.expr, &context)?;

            let stmts_len = expr_for_loop.body.stmts.len();
            let context = context.for_sub_scope_context();

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

                    walk_stmt(stmt, tail_expr_target_kind, &context)
                })
                .collect::<syn::Result<Vec<()>>>()?;

            Ok(())
        }
        Expr::Group(expr_group) => {
            let HandleInertAttrsResult {
                is_skipped: _,
                new_context: context,
            } = handle_inert_attrs(&mut expr_group.attrs, context)?;

            walk_expr(&mut expr_group.expr, &context)
        }
        Expr::If(expr_if) => {
            let HandleInertAttrsResult {
                is_skipped: _,
                new_context: context,
            } = handle_inert_attrs(&mut expr_if.attrs, context)?;

            walk_expr(&mut expr_if.cond, &context)?;

            let stmts_len = expr_if.then_branch.stmts.len();
            let context = context.for_sub_scope_context();

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

                    walk_stmt(stmt, tail_expr_target_kind, &context)
                })
                .collect::<syn::Result<Vec<()>>>()?;

            // 注意: こちらもサブスコープ
            if let Some((_, else_branch)) = expr_if.else_branch.as_mut() {
                walk_expr(else_branch, &context)?;
            }

            Ok(())
        }
        Expr::Index(expr_index) => {
            let HandleInertAttrsResult {
                is_skipped: _,
                new_context: context,
            } = handle_inert_attrs(&mut expr_index.attrs, context)?;

            walk_expr(&mut expr_index.expr, &context)?;
            walk_expr(&mut expr_index.index, &context)
        }
        Expr::Let(expr_let) => {
            let HandleInertAttrsResult {
                is_skipped: _,
                new_context: context,
            } = handle_inert_attrs(&mut expr_let.attrs, context)?;

            walk_expr(&mut expr_let.expr, &context)
        }
        Expr::Loop(expr_loop) => {
            let HandleInertAttrsResult {
                is_skipped: _,
                new_context: context,
            } = handle_inert_attrs(&mut expr_loop.attrs, context)?;

            let stmts_len = expr_loop.body.stmts.len();
            let context = context.for_sub_scope_context();

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

                    walk_stmt(stmt, tail_expr_target_kind, &context)
                })
                .collect::<syn::Result<Vec<()>>>()?;

            Ok(())
        }
        Expr::Match(expr_match) => {
            let HandleInertAttrsResult {
                is_skipped: _,
                new_context: context,
            } = handle_inert_attrs(&mut expr_match.attrs, context)?;

            walk_expr(&mut expr_match.expr, &context)?;

            let context = context.for_sub_scope_context();

            expr_match
                .arms
                .iter_mut()
                .map(|arm| {
                    if let Some((_, guard)) = arm.guard.as_mut() {
                        walk_expr(guard, &context)?;
                    }

                    match &mut *arm.body {
                        expr @ Expr::Block(_) => {
                            walk_expr(expr, &context)?;
                        }
                        expr => {
                            handle_tail_expr(expr, &context, TailExprTargetKind::BlockTailExpr)?
                        }
                    }

                    Ok(())
                })
                .collect::<syn::Result<Vec<()>>>()?;

            Ok(())
        }
        Expr::MethodCall(expr_method_call) => {
            let HandleInertAttrsResult {
                is_skipped: _,
                new_context: context,
            } = handle_inert_attrs(&mut expr_method_call.attrs, context)?;

            walk_expr(&mut expr_method_call.receiver, &context)?;
            expr_method_call
                .args
                .iter_mut()
                .map(|expr| walk_expr(expr, &context))
                .collect::<syn::Result<Vec<()>>>()?;

            Ok(())
        }
        Expr::Paren(expr_paren) => {
            let HandleInertAttrsResult {
                is_skipped: _,
                new_context: context,
            } = handle_inert_attrs(&mut expr_paren.attrs, context)?;

            walk_expr(&mut expr_paren.expr, &context)
        }
        Expr::Range(expr_range) => {
            let HandleInertAttrsResult {
                is_skipped: _,
                new_context: context,
            } = handle_inert_attrs(&mut expr_range.attrs, context)?;

            if let Some(expr_start) = expr_range.start.as_mut() {
                walk_expr(expr_start, &context)?;
            }
            if let Some(expr_end) = expr_range.end.as_mut() {
                walk_expr(expr_end, &context)?;
            }

            Ok(())
        }
        Expr::RawAddr(expr_raw_addr) => {
            let HandleInertAttrsResult {
                is_skipped: _,
                new_context: context,
            } = handle_inert_attrs(&mut expr_raw_addr.attrs, context)?;

            walk_expr(&mut expr_raw_addr.expr, &context)
        }
        Expr::Reference(expr_reference) => {
            let HandleInertAttrsResult {
                is_skipped: _,
                new_context: context,
            } = handle_inert_attrs(&mut expr_reference.attrs, context)?;

            walk_expr(&mut expr_reference.expr, &context)
        }
        Expr::Repeat(expr_repeat) => {
            let HandleInertAttrsResult {
                is_skipped: _,
                new_context: context,
            } = handle_inert_attrs(&mut expr_repeat.attrs, context)?;

            walk_expr(&mut expr_repeat.expr, &context)?;
            walk_expr(&mut expr_repeat.len, &context)
        }
        Expr::Struct(expr_struct) => {
            let HandleInertAttrsResult {
                is_skipped: _,
                new_context: context,
            } = handle_inert_attrs(&mut expr_struct.attrs, context)?;

            expr_struct
                .fields
                .iter_mut()
                .map(|field| {
                    let HandleInertAttrsResult {
                        is_skipped: _,
                        new_context: context,
                    } = handle_inert_attrs(&mut field.attrs, &context)?;

                    walk_expr(&mut field.expr, &context)
                })
                .collect::<syn::Result<Vec<()>>>()?;
            if let Some(rest) = expr_struct.rest.as_mut() {
                walk_expr(rest, &context)?;
            }

            Ok(())
        }
        Expr::TryBlock(expr_try_block) => {
            let HandleInertAttrsResult {
                is_skipped: _,
                new_context: context,
            } = handle_inert_attrs(&mut expr_try_block.attrs, context)?;

            let stmts_len = expr_try_block.block.stmts.len();
            let context = context.for_sub_scope_context();

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

                    walk_stmt(stmt, tail_expr_target_kind, &context)
                })
                .collect::<syn::Result<Vec<()>>>()?;

            Ok(())
        }
        Expr::Tuple(expr_tuple) => {
            let HandleInertAttrsResult {
                is_skipped: _,
                new_context: context,
            } = handle_inert_attrs(&mut expr_tuple.attrs, context)?;

            expr_tuple
                .elems
                .iter_mut()
                .map(|expr| walk_expr(expr, &context))
                .collect::<syn::Result<Vec<()>>>()?;

            Ok(())
        }
        Expr::Unary(expr_unary) => {
            let HandleInertAttrsResult {
                is_skipped: _,
                new_context: context,
            } = handle_inert_attrs(&mut expr_unary.attrs, context)?;

            walk_expr(&mut expr_unary.expr, &context)
        }
        Expr::Unsafe(expr_unsafe) => {
            let HandleInertAttrsResult {
                is_skipped: _,
                new_context: context,
            } = handle_inert_attrs(&mut expr_unsafe.attrs, context)?;

            let stmts_len = expr_unsafe.block.stmts.len();
            let context = context.for_sub_scope_context();

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

                    walk_stmt(stmt, tail_expr_target_kind, &context)
                })
                .collect::<syn::Result<Vec<()>>>()?;

            Ok(())
        }
        Expr::While(expr_while) => {
            let HandleInertAttrsResult {
                is_skipped: _,
                new_context: context,
            } = handle_inert_attrs(&mut expr_while.attrs, context)?;

            walk_expr(&mut expr_while.cond, &context)?;

            let stmts_len = expr_while.body.stmts.len();
            let context = context.for_sub_scope_context();

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

                    walk_stmt(stmt, tail_expr_target_kind, &context)
                })
                .collect::<syn::Result<Vec<()>>>()?;

            Ok(())
        }
        Expr::Yield(expr_yield) => {
            let HandleInertAttrsResult {
                is_skipped: _,
                new_context: context,
            } = handle_inert_attrs(&mut expr_yield.attrs, context)?;

            // nightly な機能と考えられるためテストしない

            if let Some(expr) = expr_yield.expr.as_mut() {
                walk_expr(expr, &context)?;
            }

            Ok(())
        }

        Expr::Macro(ExprMacro { attrs, mac }) => walk_macro(attrs, mac, context),

        // 以下では何もしない
        Expr::Continue(_)
        | Expr::Infer(_)
        | Expr::Lit(_)
        | Expr::Path(_)
        | Expr::Verbatim(_)
        | _ => Ok(()),
    }
}
