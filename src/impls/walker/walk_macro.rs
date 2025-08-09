use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::parse::Parse;
use syn::parse::discouraged::Speculative;
use syn::{Attribute, Macro, Token};

use crate::impls::HooqOption;
use crate::impls::option::context::PartialReplaceContext;
use crate::impls::walker::{
    HandleInertAttrsResult, handle_inert_attrs, walk_expr, walk_item, walk_stmt,
};

pub fn walk_macro(
    attrs: &mut Vec<Attribute>,
    mac: &mut Macro,
    option: &HooqOption,
    context: &PartialReplaceContext,
) -> syn::Result<()> {
    let HandleInertAttrsResult {
        is_skiped: _,
        new_context: context,
    } = handle_inert_attrs(attrs, context)?;

    if let Some(new_token_stream) = handle_token_stream(&mac.tokens, option, &context)? {
        mac.tokens = new_token_stream;
    }

    Ok(())
}

fn handle_token_stream(
    token_stream: &TokenStream,
    option: &HooqOption,
    context: &PartialReplaceContext,
) -> syn::Result<Option<TokenStream>> {
    // ここでパースできない場合はRustコードを扱わないマクロであるため諦める
    let Ok(EvaluableList(evaluables)) = syn::parse2::<EvaluableList>(token_stream.clone()) else {
        return Ok(None);
    };

    let new_stream: TokenStream = evaluables
        .into_iter()
        .map(|(eval, punct)| {
            let eval_hooked = match eval {
                Evaluable::File(file) => file
                    .items
                    .into_iter()
                    .map(|mut item| {
                        walk_item(&mut item, option, context)?;
                        Ok(item.to_token_stream())
                    })
                    .collect::<syn::Result<_>>()?,
                Evaluable::Expr(mut expr) => {
                    walk_expr(&mut expr, option, context)?;
                    expr.to_token_stream()
                }
                Evaluable::Item(mut item) => {
                    walk_item(&mut item, option, context)?;
                    item.to_token_stream()
                }
                Evaluable::Stmt(mut stmt) => {
                    // マクロ内の最後の TailExpr であったとしても、
                    // これを TailExprTargetKind::*TailExpr としてみなそうとすると
                    // 考え方が煩雑になるので諦める
                    walk_stmt(
                        &mut stmt,
                        super::TailExprTargetKind::NotTarget,
                        option,
                        context,
                    )?;
                    stmt.to_token_stream()
                }
            };

            Ok(match punct {
                Some(p) => quote! { #eval_hooked #p },
                None => eval_hooked,
            })
        })
        .collect::<syn::Result<_>>()?;

    Ok(Some(new_stream))
}

#[derive(Debug)]
enum Evaluable {
    File(syn::File),
    Item(syn::Item),
    Expr(syn::Expr),
    Stmt(syn::Stmt),
}

impl Parse for Evaluable {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let forked = input.fork();
        if let Ok(file) = forked.parse::<syn::File>()
            && (forked.peek(Token![;]) || forked.peek(Token![,]) || forked.is_empty())
        {
            input.advance_to(&forked);
            return Ok(Evaluable::File(file));
        }

        let forked = input.fork();
        if let Ok(item) = forked.parse::<syn::Item>()
            && (forked.peek(Token![;]) || forked.peek(Token![,]) || forked.is_empty())
        {
            input.advance_to(&forked);
            return Ok(Evaluable::Item(item));
        }

        let forked = input.fork();
        if let Ok(expr) = forked.parse::<syn::Expr>()
            && (forked.peek(Token![;]) || forked.peek(Token![,]) || forked.is_empty())
        {
            input.advance_to(&forked);
            return Ok(Evaluable::Expr(expr));
        }

        // TODO: セミコロンの判定取り扱いについてこのままで問題ないかを確かめるテストケースを設ける
        let forked = input.fork();
        if let Ok(stmt) = forked.parse::<syn::Stmt>()
            && (forked.peek(Token![;]) || forked.peek(Token![,]) || forked.is_empty())
        {
            input.advance_to(&forked);
            return Ok(Evaluable::Stmt(stmt));
        }

        Err(syn::Error::new(
            input.span(),
            "Failed to parse as Evaluable (File, Expr, Item, or Stmt)",
        ))
    }
}

#[derive(Debug)]
struct EvaluableList(Vec<(Evaluable, Option<TokenStream>)>);

impl Parse for EvaluableList {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut res = Vec::new();

        while let Ok(ev) = input.parse::<Evaluable>() {
            let punct = if let Ok(p) = input.parse::<Token![,]>() {
                Some(p.to_token_stream())
            } else if let Ok(p) = input.parse::<Token![;]>() {
                Some(p.to_token_stream())
            } else {
                None
            };

            res.push((ev, punct));

            if input.is_empty() {
                break;
            }
        }

        Ok(EvaluableList(res))
    }
}
