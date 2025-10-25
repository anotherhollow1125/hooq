use std::path::PathBuf;
use std::str::FromStr;

use meta_vars::{META_VARS_LIST, MetaVars};
use proc_macro2::{Group, Ident, Span, TokenStream, TokenTree};
use quote::ToTokens;
use syn::{Expr, parse_quote};

use crate::impls::inert_attr::context::HookInfo;
use crate::impls::method::Method;

mod meta_vars;

fn get_abs_path(q_span: Span) -> String {
    // Cargoプロジェクト以下の場合 local_file / file からは相対パスが返る
    let span_path = q_span.unwrap().local_file().unwrap_or_else(|| {
        let path = q_span.unwrap().file();
        PathBuf::from(path)
    });
    let cargo_path = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| String::new());
    let cargo_path = PathBuf::from(cargo_path);

    let path = if span_path.is_absolute() {
        span_path.clone()
    } else {
        cargo_path.join(&span_path)
    };
    path.to_string_lossy().to_string()
}

fn get_file_name(q_span: Span) -> String {
    let path = q_span.unwrap().file();
    let file = path.rsplit('/').next().unwrap_or(&path);

    file.to_string()
}

impl HookInfo<'_> {
    pub fn render_expr_with_method(&self, expr: &mut Expr, q_span: Span) -> syn::Result<()> {
        let mut method = TokenStream::new();

        let Method::Insert(dot, method_template) = self.method().clone() else {
            unimplemented!()
        };
        self.expand_meta_vars(method_template, &mut method, q_span)?;

        let method: TokenStream = method
            .into_iter()
            .map(|mut tt| {
                tt.set_span(q_span);
                tt
            })
            .collect();

        let new_expr: Expr = parse_quote! {
            #expr #dot #method
        };

        *expr = new_expr;

        Ok(())
    }

    // 再帰的に適用するために ts, res を引数としている
    fn expand_meta_vars(
        &self,
        ts: TokenStream,
        res: &mut TokenStream,
        q_span: Span,
    ) -> syn::Result<()> {
        let mut next_is_replace_target = false;
        for tt in ts.into_iter() {
            match tt {
                TokenTree::Punct(punct) => {
                    if next_is_replace_target {
                        return Err(syn::Error::new(punct.span(), "unexpected token after `$`"));
                    }

                    if punct.as_char() == '$' {
                        next_is_replace_target = true;
                    } else {
                        res.extend([TokenTree::Punct(punct)]);
                    }
                }
                TokenTree::Ident(ident) => {
                    if !next_is_replace_target {
                        res.extend([TokenTree::Ident(ident)]);
                        continue;
                    }

                    next_is_replace_target = false;

                    res.extend([self.meta_vars2token_stream(&ident, q_span)?]);
                }
                TokenTree::Group(group) => {
                    if next_is_replace_target {
                        return Err(syn::Error::new(group.span(), "unexpected token after `$`"));
                    }

                    let mut res_for_group = TokenStream::new();
                    self.expand_meta_vars(group.stream(), &mut res_for_group, q_span)?;

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

        Ok(())
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
        var_ident: &Ident,
        #[allow(unused)] q_span: Span,
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
            Ok(MetaVars::AbsPath) => {
                let path = get_abs_path(q_span);

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
            Ok(MetaVars::ExprStr) => {
                let expr_str = self.expr_str;

                Ok(parse_quote! {
                    #expr_str
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
            Ok(MetaVars::Bindings) => Ok(self.get_bindings_token_stream()),
            Ok(MetaVars::HooqMeta) => {
                let line = q_span.unwrap().line();
                let column = q_span.unwrap().column();
                let path = q_span.unwrap().file();
                let abs_path = get_abs_path(q_span);
                let file = get_file_name(q_span);
                let expr_str = self.expr_str;
                let count = self.get_count();
                let bindings = self.get_bindings_token_stream();

                Ok(parse_quote! {
                    ::hooq::HooqMeta {
                        line: #line,
                        column: #column,
                        path: #path,
                        abs_path: #abs_path,
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
