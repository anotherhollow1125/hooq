use std::rc::Rc;
use std::str::FromStr;

use meta_vars::{META_VARS_LIST, MetaVars};
use proc_macro2::{Group, Ident, Span, TokenStream, TokenTree};
use quote::ToTokens;
use syn::{Expr, Token, parse_quote};

use crate::impls::inert_attr::context::{HookInfo, LocalContextField};
use crate::impls::method::Method;

mod describe_expr;
mod meta_vars;

fn get_file_name(q_span: Span) -> String {
    let path = q_span.unwrap().file();
    let file = path.rsplit('/').next().unwrap_or(&path);

    file.to_string()
}

impl HookInfo<'_> {
    pub fn render_expr_with_method(
        &self,
        expr: &mut Expr,
        q_span: Span,
    ) -> syn::Result<Option<Token![!]>> {
        fn spanned(method: TokenStream, q_span: Span) -> TokenStream {
            // NOTE:
            // parse_quote_spanned! ではなぜか Span が正しく設定されない
            // そのため、イテレータを活用して設定している
            // TODO: もっと良い方法があればそちらに変更する
            let method: TokenStream = method
                .into_iter()
                .map(|mut tt| {
                    tt.set_span(q_span);
                    tt
                })
                .collect();

            method
        }

        let (new_expr, exc): (Expr, Option<Token![!]>) = match self.method().clone() {
            Method::Insert(dot, method_template, exc) => {
                let method = self.expand_meta_vars(expr, method_template, q_span, None)?;
                let method = spanned(method, q_span);

                (
                    parse_quote! {
                        #expr #dot #method
                    },
                    exc,
                )
            }
            Method::Replace(method_template, exc) => {
                /*
                // NOTE: Span を適切に設定するため、$expr の置換を後に行う
                // 以下のようなコードの構想があったが、
                // q_span で一気に書き換えても特に問題なく $expr に赤線が引かれたので、
                // 2回展開を行うのはやめることとした
                // 普段このようなコードは残さないがこちらはメモと共に残すべきと考えた

                let mut pre_method = TokenStream::new();

                self.expand_meta_vars(None, method_template, &mut pre_method, q_span)?;
                let pre_method = spanned(pre_method, q_span);

                let mut method = TokenStream::new();
                self.expand_meta_vars(Some(expr), pre_method, &mut method, q_span)?;
                */

                let method = self.expand_meta_vars(expr, method_template, q_span, None)?;
                let method = spanned(method, q_span);

                (
                    parse_quote! {
                        #method
                    },
                    exc,
                )
            }
        };

        *expr = new_expr;

        Ok(exc)
    }

    fn expand_meta_vars(
        &self,
        expr: &Expr,
        ts: TokenStream,
        q_span: Span,
        method_context: Option<Rc<LocalContextField<Method>>>,
    ) -> syn::Result<TokenStream> {
        let mut res: TokenStream = TokenStream::new();
        let mut next_is_replace_target = false;

        for tt in ts.into_iter() {
            match tt {
                TokenTree::Punct(punct) => {
                    if next_is_replace_target {
                        return Err(syn::Error::new(punct.span(), "unexpected token after `$`"));
                    }

                    match punct.as_char() {
                        '$' => next_is_replace_target = true,
                        _ => res.extend([TokenTree::Punct(punct)]),
                    }
                }
                TokenTree::Ident(ident) => {
                    if !next_is_replace_target {
                        res.extend([TokenTree::Ident(ident)]);
                        continue;
                    }

                    next_is_replace_target = false;

                    res.extend([self.meta_vars2token_stream(
                        expr,
                        &ident,
                        q_span,
                        method_context.clone(),
                    )?]);
                }
                TokenTree::Group(group) => {
                    if next_is_replace_target {
                        return Err(syn::Error::new(group.span(), "unexpected token after `$`"));
                    }

                    let res_for_group = self.expand_meta_vars(
                        expr,
                        group.stream(),
                        q_span,
                        method_context.clone(),
                    )?;

                    let new_group = Group::new(group.delimiter(), res_for_group);

                    res.extend([TokenTree::Group(new_group)]);
                }
                TokenTree::Literal(literal) => {
                    if next_is_replace_target {
                        return Err(syn::Error::new(
                            literal.span(),
                            "unexpected token after `$`",
                        ));
                    }

                    res.extend([TokenTree::Literal(literal)]);
                }
            }
        }

        Ok(res)
    }

    fn get_count(&self) -> String {
        let kind = self.kind;
        let count = self.counter().borrow().get_count(kind);
        let th = match count % 100 {
            11..=13 => "th",
            n => match n % 10 {
                1 => "st",
                2 => "nd",
                3 => "rd",
                _ => "th",
            },
        };

        format!("{count}{th} {kind}")
    }

    fn get_fn_name(&self) -> String {
        self.fn_info()
            .map(|info| info.name())
            .unwrap_or_else(|| "<unknown>".to_string())
    }

    fn get_fn_sig(&self) -> String {
        self.fn_info()
            .map(|info| info.sig())
            .unwrap_or_else(|| "<unknown>".to_string())
    }

    fn get_bindings_token_stream(&self) -> TokenStream {
        let tuples = self
            .available_bindings()
            .into_iter()
            .map(|(k, v)| -> TokenStream {
                let expr = v.to_token_stream().to_string();

                parse_quote! {
                    (::std::string::ToString::to_string(#k), {
                        let expr = ::std::string::ToString::to_string(#expr);
                        let value: ::std::rc::Rc<dyn ::std::any::Any> = ::std::rc::Rc::new(#v);

                        ::hooq::BindingPayload {
                            expr,
                            value,
                        }
                    })
                }
            });

        parse_quote! {
            ::std::collections::HashMap::from([
                #(#tuples),*
            ])
        }
    }

    fn meta_vars2token_stream(
        &self,
        expr: &Expr,
        var_ident: &Ident,
        q_span: Span,
        method_context: Option<Rc<LocalContextField<Method>>>,
    ) -> syn::Result<TokenStream> {
        match MetaVars::from_str(var_ident.to_string().as_str()) {
            Ok(MetaVars::Line) => {
                let line = q_span.unwrap().line();

                Ok(parse_quote! {
                    #line
                })
            }
            Ok(MetaVars::Column) => {
                let col = q_span.unwrap().column();

                Ok(parse_quote! {
                    #col
                })
            }
            Ok(MetaVars::Path) => {
                let path = q_span.unwrap().file();

                Ok(parse_quote! {
                    #path
                })
            }
            Ok(MetaVars::File) => {
                let file = get_file_name(q_span);

                Ok(parse_quote! {
                    #file
                })
            }
            Ok(MetaVars::Expr) => Ok(expr.to_token_stream()),
            Ok(MetaVars::ExprStr) => {
                let expr_str = self.source_tokenstream.to_string();

                Ok(parse_quote! {
                    #expr_str
                })
            }
            Ok(MetaVars::ExprStrShort) => {
                let expr_str_short = describe_expr::describe_expr_short(expr, self.kind);

                Ok(parse_quote! {
                    #expr_str_short
                })
            }
            Ok(MetaVars::ExprStrShortOneLine) => {
                let expr_str_short_oneline: String =
                    describe_expr::describe_expr_short(expr, self.kind)
                        .lines()
                        .map(|line| line.trim())
                        .collect::<Vec<_>>()
                        .join(" ");

                Ok(parse_quote! {
                    #expr_str_short_oneline
                })
            }
            Ok(MetaVars::Source) => {
                let source_ts = self.source_tokenstream.clone();

                Ok(parse_quote! {
                    #source_ts
                })
            }
            Ok(MetaVars::Count) => {
                let val = self.get_count();

                Ok(parse_quote! {
                    #val
                })
            }
            Ok(MetaVars::FnName) => {
                let fn_name = self.get_fn_name();

                Ok(parse_quote! {
                    #fn_name
                })
            }
            Ok(MetaVars::FnSig) => {
                let fn_sig = self.get_fn_sig();

                Ok(parse_quote! {
                    #fn_sig
                })
            }
            Ok(MetaVars::SoFar) => {
                let Some(ancestor) = (match method_context {
                    Some(current) => current.get_overridden_ancestor(),
                    None => self.hook_context.get_overridden_ancestor_of_method(),
                }) else {
                    return Err(syn::Error::new(
                        var_ident.span(),
                        "`$so_far` used but no overridden method found.",
                    ));
                };

                let (Method::Insert(_, ts, _) | Method::Replace(ts, _)) = &**ancestor;

                let ancestor_method =
                    self.expand_meta_vars(expr, ts.clone(), q_span, Some(ancestor))?;

                Ok(ancestor_method)
            }
            Ok(MetaVars::Bindings) => Ok(self.get_bindings_token_stream()),
            Ok(MetaVars::HooqMeta) => {
                let line = q_span.unwrap().line();
                let column = q_span.unwrap().column();
                let path = q_span.unwrap().file();
                let file = get_file_name(q_span);
                let expr_str = self.source_tokenstream.to_string();
                let count = self.get_count();
                let bindings = self.get_bindings_token_stream();

                Ok(parse_quote! {
                    ::hooq::HooqMeta {
                        line: #line,
                        column: #column,
                        path: #path,
                        file: #file,
                        expr_str: #expr_str,
                        count: #count,
                        bindings: #bindings,
                    }
                })
            }
            Err(binding) => {
                let Some(expr) = self.get_binding(&binding) else {
                    return Err(syn::Error::new(
                        var_ident.span(),
                        format!(
                            "unknown meta variable or binding: ${var_ident}
available bindings:
{}
available meta variables:
{}",
                            self.available_bindings()
                                .into_iter()
                                .map(|(k, _v)| format!(" - ${k}"))
                                .collect::<Vec<_>>()
                                .join("\n"),
                            META_VARS_LIST
                                .iter()
                                .map(|m| format!(" - ${m}"))
                                .collect::<Vec<_>>()
                                .join("\n"),
                        ),
                    ));
                };

                Ok(expr.to_token_stream())
            }
        }
    }
}
