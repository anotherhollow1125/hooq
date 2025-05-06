use crate::impls::option::context::{PartialReplaceContext, ReplaceKind};
use proc_macro2::Span;
use quote::ToTokens;
use syn::spanned::Spanned;
use syn::{
    Attribute, Expr, ExprCall, ExprPath, Item, ItemMod, Local, LocalInit, Stmt, parse_quote,
};

use crate::impls::inert_attr::{InertAttrOption, extract_hooq_info_from_attrs};
pub use crate::impls::option::HooqOption;
use crate::impls::utils::get_attrs_from_expr;

pub fn walk_stmt(
    stmt: &mut Stmt,
    is_top_level: bool,
    is_tail: bool,
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
                is_skiped,
                new_context: context,
            } = handle_inert_attrs(attrs, context)?;
            if is_skiped {
                return Ok(());
            }

            walk_expr(expr, option, &context)?;
            if let Some((_, expr_else)) = diverge {
                walk_expr(expr_else, option, &context)?;
            }

            Ok(())
        }
        Stmt::Item(item) => walk_item(item, option, context),

        // 以下は返り値となる Expr
        // 次の場合にフックすることにしたい
        // - #[hooq] をつけた関数のトップレベルの時: 常にフック
        // - 上記以外: `Ok` | `Err` の時にフック
        Stmt::Expr(expr, None) if is_tail => {
            let Some(attrs) = get_attrs_from_expr(expr) else {
                return Ok(());
            };
            let HandleInertAttrsResult {
                is_skiped,
                new_context: context,
            } = handle_inert_attrs(attrs, context)?;
            if is_skiped {
                return Ok(());
            }

            walk_expr(expr, option, &context)?;

            // トップレベルは常にフック
            if is_top_level {
                let q_span = expr.span();

                // TODO: tag の None を適切なものに
                replace_expr(expr, ReplaceKind::TailExpr, q_span, option, &context)?;

                return Ok(());
            }

            // Ok or Err の時にフック
            let Expr::Call(ExprCall { func, .. }) = expr else {
                return Ok(());
            };

            let Expr::Path(ExprPath { path, .. }) = *func.clone() else {
                return Ok(());
            };

            if path.is_ident("Ok") || path.is_ident("Err") {
                let q_span = path.span();

                // TODO: tag の None を適切なものに
                replace_expr(expr, ReplaceKind::TailExpr, q_span, option, &context)?;

                return Ok(());
            }

            Ok(())
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
    attrs: &mut [Attribute],
    context: &'a PartialReplaceContext,
) -> syn::Result<HandleInertAttrsResult<'a>> {
    let InertAttrOption {
        is_skiped,
        tag,
        method,
    } = extract_hooq_info_from_attrs(attrs)?;

    Ok(HandleInertAttrsResult {
        is_skiped,
        new_context: PartialReplaceContext::new(context, tag, method),
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
                is_skiped,
                new_context: context,
            } = handle_inert_attrs(&mut item_fn.attrs, context)?;
            if is_skiped {
                return Ok(());
            }

            let stmts_len = item_fn.block.stmts.len();
            item_fn
                .block
                .stmts
                .iter_mut()
                .enumerate()
                .map(|(i, stmt)| walk_stmt(stmt, false, i == stmts_len - 1, option, &context))
                .collect::<syn::Result<Vec<()>>>()?;

            Ok(())
        }
        Item::Impl(item_impl) => {
            let HandleInertAttrsResult {
                is_skiped,
                new_context: context,
            } = handle_inert_attrs(&mut item_impl.attrs, context)?;
            if is_skiped {
                return Ok(());
            }

            item_impl
                .items
                .iter_mut()
                .map(|impl_item| {
                    match impl_item {
                        syn::ImplItem::Fn(impl_item_fn) => {
                            let stmts_len = impl_item_fn.block.stmts.len();
                            impl_item_fn
                                .block
                                .stmts
                                .iter_mut()
                                .enumerate()
                                .map(|(i, stmt)| {
                                    walk_stmt(stmt, false, i == stmts_len - 1, option, &context)
                                })
                                .collect::<syn::Result<Vec<()>>>()?;

                            Ok(())
                        }

                        // 以下の場合何もしない
                        syn::ImplItem::Const(_)
                        | syn::ImplItem::Type(_)
                        | syn::ImplItem::Macro(_)
                        | syn::ImplItem::Verbatim(_)
                        | _ => Ok(()),
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
                is_skiped,
                new_context: context,
            } = handle_inert_attrs(attrs, context)?;
            if is_skiped {
                return Ok(());
            }

            items
                .iter_mut()
                .map(|item| walk_item(item, option, &context))
                .collect::<syn::Result<Vec<()>>>()?;

            Ok(())
        }

        // 以下では何もしない
        // Item::TraitAlias は将来のために予約された要素
        Item::Const(_)
        | Item::Enum(_)
        | Item::ExternCrate(_)
        | Item::ForeignMod(_)
        | Item::Macro(_)
        | Item::Mod(ItemMod { content: None, .. })
        | Item::Static(_)
        | Item::Struct(_)
        | Item::Trait(_)
        | Item::TraitAlias(_)
        | Item::Type(_)
        | Item::Union(_)
        | Item::Use(_)
        | Item::Verbatim(_)
        | _ => Ok(()),
    }
}

fn replace_expr(
    expr_field: &mut Expr,
    kind: ReplaceKind,
    q_span: Span,
    option: &HooqOption,
    context: &PartialReplaceContext,
) -> syn::Result<()> {
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
            if is_skiped {
                return Ok(());
            }

            walk_expr(&mut expr_try.expr, option, &context)?;

            let q_span = expr_try.question_token.span();

            // TODO: tag の None を適切なものに
            replace_expr(
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
            if is_skiped {
                return Ok(());
            }

            if let Some(expr) = expr_return.expr.as_mut() {
                walk_expr(expr, option, &context)?;

                let q_span = expr_return.return_token.span();

                // TODO: tag の None を適切なものに
                replace_expr(expr, ReplaceKind::Return, q_span, option, &context)?;
            }

            Ok(())
        }

        // ネストの中身を見る必要があるバリアント
        Expr::Array(expr_array) => {
            let HandleInertAttrsResult {
                is_skiped,
                new_context: context,
            } = handle_inert_attrs(&mut expr_array.attrs, context)?;
            if is_skiped {
                return Ok(());
            }

            expr_array
                .elems
                .iter_mut()
                .map(|expr| walk_expr(expr, option, &context))
                .collect::<syn::Result<Vec<()>>>()?;

            Ok(())
        }
        Expr::Assign(expr_assign) => {
            let HandleInertAttrsResult {
                is_skiped,
                new_context: context,
            } = handle_inert_attrs(&mut expr_assign.attrs, context)?;
            if is_skiped {
                return Ok(());
            }

            walk_expr(&mut expr_assign.right, option, &context)
        }
        Expr::Async(expr_async) => {
            let HandleInertAttrsResult {
                is_skiped,
                new_context: context,
            } = handle_inert_attrs(&mut expr_async.attrs, context)?;
            if is_skiped {
                return Ok(());
            }

            let stmts_len = expr_async.block.stmts.len();
            expr_async
                .block
                .stmts
                .iter_mut()
                .enumerate()
                .map(|(i, stmt)| walk_stmt(stmt, false, i == stmts_len - 1, option, &context))
                .collect::<syn::Result<Vec<()>>>()?;

            Ok(())
        }
        Expr::Await(expr_await) => {
            let HandleInertAttrsResult {
                is_skiped,
                new_context: context,
            } = handle_inert_attrs(&mut expr_await.attrs, context)?;
            if is_skiped {
                return Ok(());
            }

            walk_expr(&mut expr_await.base, option, &context)
        }
        Expr::Binary(expr_binary) => {
            let HandleInertAttrsResult {
                is_skiped,
                new_context: context,
            } = handle_inert_attrs(&mut expr_binary.attrs, context)?;
            if is_skiped {
                return Ok(());
            }

            walk_expr(&mut expr_binary.left, option, &context)?;
            walk_expr(&mut expr_binary.right, option, &context)
        }
        Expr::Block(expr_block) => {
            let HandleInertAttrsResult {
                is_skiped,
                new_context: context,
            } = handle_inert_attrs(&mut expr_block.attrs, context)?;
            if is_skiped {
                return Ok(());
            }

            let stmts_len = expr_block.block.stmts.len();
            expr_block
                .block
                .stmts
                .iter_mut()
                .enumerate()
                .map(|(i, stmt)| walk_stmt(stmt, false, i == stmts_len - 1, option, &context))
                .collect::<syn::Result<Vec<()>>>()?;

            Ok(())
        }
        Expr::Break(expr_break) => {
            let HandleInertAttrsResult {
                is_skiped,
                new_context: context,
            } = handle_inert_attrs(&mut expr_break.attrs, context)?;
            if is_skiped {
                return Ok(());
            }

            if let Some(res) = expr_break.expr.as_mut() {
                walk_expr(res, option, &context)?;
            }

            Ok(())
        }
        Expr::Call(expr_call) => {
            let HandleInertAttrsResult {
                is_skiped,
                new_context: context,
            } = handle_inert_attrs(&mut expr_call.attrs, context)?;
            if is_skiped {
                return Ok(());
            }

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
                is_skiped,
                new_context: context,
            } = handle_inert_attrs(&mut expr_cast.attrs, context)?;
            if is_skiped {
                return Ok(());
            }

            walk_expr(&mut expr_cast.expr, option, &context)
        }
        Expr::Closure(expr_closure) => {
            let HandleInertAttrsResult {
                is_skiped,
                new_context: context,
            } = handle_inert_attrs(&mut expr_closure.attrs, context)?;
            if is_skiped {
                return Ok(());
            }

            walk_expr(&mut expr_closure.body, option, &context)
        }
        Expr::Const(expr_const) => {
            let HandleInertAttrsResult {
                is_skiped,
                new_context: context,
            } = handle_inert_attrs(&mut expr_const.attrs, context)?;
            if is_skiped {
                return Ok(());
            }

            let stmts_len = expr_const.block.stmts.len();
            expr_const
                .block
                .stmts
                .iter_mut()
                .enumerate()
                .map(|(i, stmt)| walk_stmt(stmt, false, i == stmts_len - 1, option, &context))
                .collect::<syn::Result<Vec<()>>>()?;

            Ok(())
        }
        Expr::Field(expr_field) => {
            let HandleInertAttrsResult {
                is_skiped,
                new_context: context,
            } = handle_inert_attrs(&mut expr_field.attrs, context)?;
            if is_skiped {
                return Ok(());
            }

            walk_expr(&mut expr_field.base, option, &context)
        }
        Expr::ForLoop(expr_for_loop) => {
            let HandleInertAttrsResult {
                is_skiped,
                new_context: context,
            } = handle_inert_attrs(&mut expr_for_loop.attrs, context)?;
            if is_skiped {
                return Ok(());
            }

            walk_expr(&mut expr_for_loop.expr, option, &context)?;
            let stmts_len = expr_for_loop.body.stmts.len();
            expr_for_loop
                .body
                .stmts
                .iter_mut()
                .enumerate()
                .map(|(i, stmt)| walk_stmt(stmt, false, i == stmts_len - 1, option, &context))
                .collect::<syn::Result<Vec<()>>>()?;

            Ok(())
        }
        Expr::Group(expr_group) => {
            let HandleInertAttrsResult {
                is_skiped,
                new_context: context,
            } = handle_inert_attrs(&mut expr_group.attrs, context)?;
            if is_skiped {
                return Ok(());
            }

            walk_expr(&mut expr_group.expr, option, &context)
        }
        Expr::If(expr_if) => {
            let HandleInertAttrsResult {
                is_skiped,
                new_context: context,
            } = handle_inert_attrs(&mut expr_if.attrs, context)?;
            if is_skiped {
                return Ok(());
            }

            walk_expr(&mut expr_if.cond, option, &context)?;
            let stmts_len = expr_if.then_branch.stmts.len();
            expr_if
                .then_branch
                .stmts
                .iter_mut()
                .enumerate()
                .map(|(i, stmt)| walk_stmt(stmt, false, i == stmts_len - 1, option, &context))
                .collect::<syn::Result<Vec<()>>>()?;

            if let Some((_, else_branch)) = expr_if.else_branch.as_mut() {
                walk_expr(else_branch, option, &context)?;
            }

            Ok(())
        }
        Expr::Index(expr_index) => {
            let HandleInertAttrsResult {
                is_skiped,
                new_context: context,
            } = handle_inert_attrs(&mut expr_index.attrs, context)?;
            if is_skiped {
                return Ok(());
            }

            walk_expr(&mut expr_index.expr, option, &context)?;
            walk_expr(&mut expr_index.index, option, &context)
        }
        Expr::Let(expr_let) => {
            let HandleInertAttrsResult {
                is_skiped,
                new_context: context,
            } = handle_inert_attrs(&mut expr_let.attrs, context)?;
            if is_skiped {
                return Ok(());
            }

            // ↑おそらく将来のために予約...？
            walk_expr(&mut expr_let.expr, option, &context)
        }
        Expr::Loop(expr_loop) => {
            let HandleInertAttrsResult {
                is_skiped,
                new_context: context,
            } = handle_inert_attrs(&mut expr_loop.attrs, context)?;
            if is_skiped {
                return Ok(());
            }

            let stmts_len = expr_loop.body.stmts.len();
            expr_loop
                .body
                .stmts
                .iter_mut()
                .enumerate()
                .map(|(i, stmt)| walk_stmt(stmt, false, i == stmts_len - 1, option, &context))
                .collect::<syn::Result<Vec<()>>>()?;

            Ok(())
        }
        Expr::Match(expr_match) => {
            let HandleInertAttrsResult {
                is_skiped,
                new_context: context,
            } = handle_inert_attrs(&mut expr_match.attrs, context)?;
            if is_skiped {
                return Ok(());
            }

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
                is_skiped,
                new_context: context,
            } = handle_inert_attrs(&mut expr_method_call.attrs, context)?;
            if is_skiped {
                return Ok(());
            }

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
                is_skiped,
                new_context: context,
            } = handle_inert_attrs(&mut expr_paren.attrs, context)?;
            if is_skiped {
                return Ok(());
            }

            walk_expr(&mut expr_paren.expr, option, &context)
        }
        Expr::Range(expr_range) => {
            let HandleInertAttrsResult {
                is_skiped,
                new_context: context,
            } = handle_inert_attrs(&mut expr_range.attrs, context)?;
            if is_skiped {
                return Ok(());
            }

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
                is_skiped,
                new_context: context,
            } = handle_inert_attrs(&mut expr_raw_addr.attrs, context)?;
            if is_skiped {
                return Ok(());
            }

            walk_expr(&mut expr_raw_addr.expr, option, &context)
        }
        Expr::Reference(expr_reference) => {
            let HandleInertAttrsResult {
                is_skiped,
                new_context: context,
            } = handle_inert_attrs(&mut expr_reference.attrs, context)?;
            if is_skiped {
                return Ok(());
            }

            walk_expr(&mut expr_reference.expr, option, &context)
        }
        Expr::Repeat(expr_repeat) => {
            let HandleInertAttrsResult {
                is_skiped,
                new_context: context,
            } = handle_inert_attrs(&mut expr_repeat.attrs, context)?;
            if is_skiped {
                return Ok(());
            }

            walk_expr(&mut expr_repeat.expr, option, &context)?;
            walk_expr(&mut expr_repeat.len, option, &context)
        }
        Expr::Struct(expr_struct) => {
            let HandleInertAttrsResult {
                is_skiped,
                new_context: context,
            } = handle_inert_attrs(&mut expr_struct.attrs, context)?;
            if is_skiped {
                return Ok(());
            }

            expr_struct
                .fields
                .iter_mut()
                .map(|field| {
                    walk_expr(&mut field.expr, option, &context)?;

                    Ok(())
                })
                .collect::<syn::Result<Vec<()>>>()?;
            if let Some(rest) = expr_struct.rest.as_mut() {
                walk_expr(rest, option, &context)?;
            }

            Ok(())
        }
        Expr::TryBlock(expr_try_block) => {
            let HandleInertAttrsResult {
                is_skiped,
                new_context: context,
            } = handle_inert_attrs(&mut expr_try_block.attrs, context)?;
            if is_skiped {
                return Ok(());
            }

            let stmts_len = expr_try_block.block.stmts.len();
            expr_try_block
                .block
                .stmts
                .iter_mut()
                .enumerate()
                .map(|(i, stmt)| walk_stmt(stmt, false, i == stmts_len - 1, option, &context))
                .collect::<syn::Result<Vec<()>>>()?;

            Ok(())
        }
        Expr::Tuple(expr_tuple) => {
            let HandleInertAttrsResult {
                is_skiped,
                new_context: context,
            } = handle_inert_attrs(&mut expr_tuple.attrs, context)?;
            if is_skiped {
                return Ok(());
            }

            expr_tuple
                .elems
                .iter_mut()
                .map(|expr| walk_expr(expr, option, &context))
                .collect::<syn::Result<Vec<()>>>()?;

            Ok(())
        }
        Expr::Unary(expr_unary) => {
            let HandleInertAttrsResult {
                is_skiped,
                new_context: context,
            } = handle_inert_attrs(&mut expr_unary.attrs, context)?;
            if is_skiped {
                return Ok(());
            }

            walk_expr(&mut expr_unary.expr, option, &context)
        }
        Expr::Unsafe(expr_unsafe) => {
            let HandleInertAttrsResult {
                is_skiped,
                new_context: context,
            } = handle_inert_attrs(&mut expr_unsafe.attrs, context)?;
            if is_skiped {
                return Ok(());
            }

            let stmts_len = expr_unsafe.block.stmts.len();
            expr_unsafe
                .block
                .stmts
                .iter_mut()
                .enumerate()
                .map(|(i, stmt)| walk_stmt(stmt, false, i == stmts_len - 1, option, &context))
                .collect::<syn::Result<Vec<()>>>()?;

            Ok(())
        }
        Expr::While(expr_while) => {
            let HandleInertAttrsResult {
                is_skiped,
                new_context: context,
            } = handle_inert_attrs(&mut expr_while.attrs, context)?;
            if is_skiped {
                return Ok(());
            }

            walk_expr(&mut expr_while.cond, option, &context)?;
            let stmts_len = expr_while.body.stmts.len();
            expr_while
                .body
                .stmts
                .iter_mut()
                .enumerate()
                .map(|(i, stmt)| walk_stmt(stmt, false, i == stmts_len - 1, option, &context))
                .collect::<syn::Result<Vec<()>>>()?;

            Ok(())
        }
        Expr::Yield(expr_yield) => {
            let HandleInertAttrsResult {
                is_skiped,
                new_context: context,
            } = handle_inert_attrs(&mut expr_yield.attrs, context)?;
            if is_skiped {
                return Ok(());
            }

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
