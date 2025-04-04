use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
    parse_quote, Attribute, Expr, ExprCall, Item, ItemFn, ItemMod, Local, LocalInit, Meta, Stmt, ExprPath
};
use syn::spanned::Spanned;

pub mod option;
pub mod utils;

pub use option::HooqOption;
use utils::{get_attrs_from_expr, strip_attr};

pub fn hooq_impls(mut f: ItemFn) -> syn::Result<TokenStream> {
    let hooq_option = HooqOption::new_from_attrs(&mut f.attrs)?;

    f.block.stmts.iter_mut().for_each(|stmt| handle_stmt(stmt, true, &hooq_option));

    Ok(quote! {
        #f
    })
}

fn handle_stmt(stmt: &mut Stmt, is_top_level: bool, option: &HooqOption) {
    match stmt {
        Stmt::Local(Local {
            attrs,
            init: Some(LocalInit { expr, diverge, .. }),
            ..
        }) => {
            if check_skip(attrs) {
                return;
            }

            handle_expr(expr, option);
            if let Some((_, expr_else)) = diverge {
                handle_expr(expr_else, option);
            }
        }
        Stmt::Item(item) => handle_item(item, option),
        Stmt::Expr(expr, Some(_)) => handle_expr(expr, option),

        // 以下は返り値となる Expr
        // 次の場合にフックすることにしたい
        // - #[hooq] をつけた関数のトップレベルの時: 常にフック
        // - 上記以外: `Ok` | `Err` の時にフック
        Stmt::Expr(expr, None) => {
            let Some(attrs) = get_attrs_from_expr(expr) else {
                return;
            };
            if check_skip(attrs) {
                return;
            }

            // トップレベルは常にフック
            if is_top_level {
                let q_span = expr.span();

                handle_expr(expr, option);
                replace_expr(expr, q_span, option);

                return;
            }

            // Ok or Err の時にフック
            let Expr::Call(ExprCall { func, .. }) = expr else {
                return;
            };

            let Expr::Path(ExprPath { path, .. }) = *func.clone() else {
                return;
            };

            if path.is_ident("Ok") || path.is_ident("Err") {
                let q_span = path.span();

                handle_expr(expr, option);
                replace_expr(expr, q_span, option);

                return;
            }            

            handle_expr(expr, option);
        }

        // 以下では何もしない
        Stmt::Local(Local { init: None, .. }) | Stmt::Macro(_) => {}
    }
}

fn check_skip(attrs: &mut [Attribute]) -> bool {
    let hooq_skip = parse_quote!(hooq::skip);

    for attr in attrs.iter_mut() {
        let Meta::Path(path) = &attr.meta else {
            continue;
        };

        if path == &hooq_skip {
            strip_attr(attr);

            return true;
        }
    }

    false
}

// 以降、本来的にはVisitやFoldで実装するべきかもしれないが、
// 網羅性の確認のため一旦全て明示的に実装した
// テストを用意した上でコミット後、Visitに変更予定

fn handle_item(item: &mut Item, option: &HooqOption) {
    match item {
        Item::Fn(item_fn) => {
            if check_skip(&mut item_fn.attrs) {
                return;
            }

            item_fn.block.stmts.iter_mut().for_each(|stmt| handle_stmt(stmt, false, option));
        }
        Item::Impl(item_impl) => {
            if check_skip(&mut item_impl.attrs) {
                return;
            }

            item_impl.items.iter_mut().for_each(|impl_item| {
                match impl_item {
                    syn::ImplItem::Fn(impl_item_fn) => {
                        impl_item_fn.block.stmts.iter_mut().for_each(|stmt| handle_stmt(stmt, false, option));
                    }

                    // 以下の場合何もしない
                    syn::ImplItem::Const(_)
                    | syn::ImplItem::Type(_)
                    | syn::ImplItem::Macro(_)
                    | syn::ImplItem::Verbatim(_)
                    | _ => {}
                }
            });
        }
        Item::Mod(ItemMod {
            attrs,
            content: Some((_, items)),
            ..
        }) => {
            if check_skip(attrs) {
                return;
            }

            items.iter_mut().for_each(|item| handle_item(item, option));
        }

        // 以下では何もしない
        Item::Const(_)
        | Item::Enum(_)
        | Item::ExternCrate(_)
        | Item::ForeignMod(_)
        | Item::Macro(_)
        | Item::Mod(ItemMod { content: None, .. })
        | Item::Static(_)
        | Item::Struct(_)
        | Item::Trait(_)
        | Item::TraitAlias(_) // 将来のために予約された要素
        | Item::Type(_)
        | Item::Union(_)
        | Item::Use(_)
        | Item::Verbatim(_)
        | _ => {}
    }
}

fn replace_expr(expr_field: &mut Expr, q_span: Span, option: &HooqOption) {
    let method = option.generate_method(q_span);
    let original_expr = expr_field.clone();

    *expr_field = parse_quote! {
        #original_expr #method
    };
}

fn handle_expr(expr: &mut Expr, option: &HooqOption) {
    match expr {
        // 置換対象となるバリアント
        Expr::Try(expr_try) => {
            if check_skip(&mut expr_try.attrs) {
                return;
            }

            handle_expr(&mut expr_try.expr, option);

            let q_span = expr_try.question_token.span();

            replace_expr(&mut expr_try.expr, q_span, option);
        },
        Expr::Return(expr_return) => {
            if check_skip(&mut expr_return.attrs) {
                return;
            }

            if let Some(expr) = expr_return.expr.as_mut() {
                handle_expr(expr, option);

                let q_span = expr_return.return_token.span();

                replace_expr(expr, q_span, option);
            }
        },

        // ネストの中身を見る必要があるバリアント
        Expr::Array(expr_array) => {
            if check_skip(&mut expr_array.attrs) {
                return;
            }

            expr_array.elems.iter_mut().for_each(|expr| handle_expr(expr, option));
        }
        Expr::Assign(expr_assign) => {
            if check_skip(&mut expr_assign.attrs) {
                return;
            }

            handle_expr(&mut expr_assign.right, option);
        }
        Expr::Async(expr_async) => {
            if check_skip(&mut expr_async.attrs) {
                return;
            }

            expr_async.block.stmts.iter_mut().for_each(|stmt| handle_stmt(stmt, false, option));
        }
        Expr::Await(expr_await) => {
            if check_skip(&mut expr_await.attrs) {
                return;
            }

            handle_expr(&mut expr_await.base, option);
        }
        Expr::Binary(expr_binary) => {
            if check_skip(&mut expr_binary.attrs) {
                return;
            }

            handle_expr(&mut expr_binary.left, option);
            handle_expr(&mut expr_binary.right, option);
        }
        Expr::Block(expr_block) => {
            if check_skip(&mut expr_block.attrs) {
                return;
            }

            expr_block.block.stmts.iter_mut().for_each(|stmt| handle_stmt(stmt, false, option));
        }
        Expr::Break(expr_break) => {
            if check_skip(&mut expr_break.attrs) {
                return;
            }

            if let Some(res) = expr_break.expr.as_mut() {
                handle_expr(res, option);
            }
        }
        Expr::Call(expr_call) => {
            if check_skip(&mut expr_call.attrs) {
                return;
            }

            handle_expr(&mut expr_call.func, option);
            expr_call.args.iter_mut().for_each(|expr| handle_expr(expr, option));
        }
        Expr::Cast(expr_cast) => {
            if check_skip(&mut expr_cast.attrs) {
                return;
            }

            handle_expr(&mut expr_cast.expr, option);
        }
        Expr::Closure(expr_closure) => {
            if check_skip(&mut expr_closure.attrs) {
                return;
            }

            handle_expr(&mut expr_closure.body, option);
        }
        Expr::Const(expr_const) => {
            if check_skip(&mut expr_const.attrs) {
                return;
            }

            expr_const.block.stmts.iter_mut().for_each(|stmt| handle_stmt(stmt, false, option));
        }
        Expr::Field(expr_field) => {
            if check_skip(&mut expr_field.attrs) {
                return;
            }

            handle_expr(&mut expr_field.base, option);
        }
        Expr::ForLoop(expr_for_loop) => {
            if check_skip(&mut expr_for_loop.attrs) {
                return;
            }

            handle_expr(&mut expr_for_loop.expr, option);
            expr_for_loop.body.stmts.iter_mut().for_each(|stmt| handle_stmt(stmt, false, option));
        }
        Expr::Group(expr_group) => {
            if check_skip(&mut expr_group.attrs) {
                return;
            }

            handle_expr(&mut expr_group.expr, option);
        }
        Expr::If(expr_if) => {
            if check_skip(&mut expr_if.attrs) {
                return;
            }

            handle_expr(&mut expr_if.cond, option);
            expr_if.then_branch.stmts.iter_mut().for_each(|stmt| handle_stmt(stmt, false, option));
        }
        Expr::Index(expr_index) => {
            if check_skip(&mut expr_index.attrs) {
                return;
            }

            handle_expr(&mut expr_index.expr, option);
            handle_expr(&mut expr_index.index, option);
        }
        Expr::Let(expr_let) => {
            if check_skip(&mut expr_let.attrs) {
                return;
            }

            // ↑おそらく将来のために予約...？
            handle_expr(&mut expr_let.expr, option);
        }
        Expr::Loop(expr_loop) => {
            if check_skip(&mut expr_loop.attrs) {
                return;
            }

            expr_loop.body.stmts.iter_mut().for_each(|stmt| handle_stmt(stmt, false, option));
        }
        Expr::Match(expr_match) => {
            if check_skip(&mut expr_match.attrs) {
                return;
            }

            handle_expr(&mut expr_match.expr, option);
            expr_match.arms.iter_mut().for_each(|arm| {
                if let Some((_, guard)) = arm.guard.as_mut() {
                    handle_expr(guard, option);
                }
                handle_expr(&mut arm.body, option);
            });
        }
        Expr::MethodCall(expr_method_call) => {
            if check_skip(&mut expr_method_call.attrs) {
                return;
            }

            handle_expr(&mut expr_method_call.receiver, option);
            expr_method_call.args.iter_mut().for_each(|expr| handle_expr(expr, option));
        }
        Expr::Paren(expr_paren) => {
            if check_skip(&mut expr_paren.attrs) {
                return;
            }

            handle_expr(&mut expr_paren.expr, option);
        }
        Expr::Range(expr_range) => {
            if check_skip(&mut expr_range.attrs) {
                return;
            }

            if let Some(expr_start) = expr_range.start.as_mut() {
                handle_expr(expr_start, option);
            }
            if let Some(expr_end) = expr_range.end.as_mut() {
                handle_expr(expr_end, option);
            }
        }
        Expr::RawAddr(expr_raw_addr) => {
            if check_skip(&mut expr_raw_addr.attrs) {
                return;
            }

            handle_expr(&mut expr_raw_addr.expr, option);
        }
        Expr::Reference(expr_reference) => {
            if check_skip(&mut expr_reference.attrs) {
                return;
            }

            handle_expr(&mut expr_reference.expr, option);
        }
        Expr::Repeat(expr_repeat) => {
            if check_skip(&mut expr_repeat.attrs) {
                return;
            }

            handle_expr(&mut expr_repeat.expr, option);
            handle_expr(&mut expr_repeat.len, option);
        }
        Expr::Struct(expr_struct) => {
            if check_skip(&mut expr_struct.attrs) {
                return;
            }

            expr_struct.fields.iter_mut().for_each(|field| {
                handle_expr(&mut field.expr, option);
            });
            if let Some(rest) = expr_struct.rest.as_mut() {
                handle_expr(rest, option);
            }
        }
        Expr::TryBlock(expr_try_block) => {
            if check_skip(&mut expr_try_block.attrs) {
                return;
            }

            expr_try_block.block.stmts.iter_mut().for_each(|stmt| handle_stmt(stmt, false, option));
        }
        Expr::Tuple(expr_tuple) => {
            if check_skip(&mut expr_tuple.attrs) {
                return;
            }

            expr_tuple.elems.iter_mut().for_each(|expr| handle_expr(expr, option));
        }
        Expr::Unary(expr_unary) => {
            if check_skip(&mut expr_unary.attrs) {
                return;
            }

            handle_expr(&mut expr_unary.expr, option);
        }
        Expr::Unsafe(expr_unsafe) => {
            if check_skip(&mut expr_unsafe.attrs) {
                return;
            }

            expr_unsafe.block.stmts.iter_mut().for_each(|stmt| handle_stmt(stmt, false, option));
        }
        Expr::While(expr_while) => {
            if check_skip(&mut expr_while.attrs) {
                return;
            }

            handle_expr(&mut expr_while.cond, option);
            expr_while.body.stmts.iter_mut().for_each(|stmt| handle_stmt(stmt, false, option));
        }
        Expr::Yield(expr_yield) => {
            if check_skip(&mut expr_yield.attrs) {
                return;
            }

            if let Some(expr) = expr_yield.expr.as_mut() {
                handle_expr(expr, option);
            }
        }
        
        // 以下では何もしない
        Expr::Continue(_)
        | Expr::Infer(_)
        | Expr::Lit(_)
        | Expr::Macro(_) // 欲を言えばこの中も見たいが、独自文法となってしまっているマクロもあるので諦める
        | Expr::Path(_)
        | Expr::Verbatim(_)
        | _ => {}
    }
}
