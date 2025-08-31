use std::path::PathBuf;
use std::str::FromStr;

use meta_vars::{META_VARS_LIST, MetaVars};
use proc_macro2::{Group, Ident, Span, TokenStream, TokenTree};
use quote::ToTokens;
use syn::parse_quote;

use crate::impls::inert_attr::context::HookInfo;

mod meta_vars;

impl HookInfo<'_> {
    pub fn generate_method(&self, q_span: Span) -> syn::Result<TokenStream> {
        let mut res = TokenStream::new();

        let method = match self.method() {
            Some(method) => method.clone(),
            None => default_method(),
        };
        self.expand_meta_vars(method, &mut res, q_span)?;

        let res = res
            .into_iter()
            .map(|mut tt| {
                tt.set_span(q_span);
                tt
            })
            .collect();

        Ok(res)
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
                        return Err(syn::Error::new(punct.span(), "Unexpected token after `$`"));
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
                        return Err(syn::Error::new(group.span(), "Unexpected token after `$`"));
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
                            "Unexpected token after `$`",
                        ));
                    }

                    res.extend([TokenTree::Literal(literal)]);
                }
            }
        }

        Ok(())
    }

    fn meta_vars2token_stream(
        &self,
        var_ident: &Ident,
        #[allow(unused)] q_span: Span,
    ) -> syn::Result<TokenStream> {
        match MetaVars::from_str(var_ident.to_string().as_str()) {
            Ok(MetaVars::Line) => {
                let line: TokenStream = {
                    let line = q_span.unwrap().line();

                    parse_quote! {
                        #line
                    }
                };

                Ok(line)
            }
            Ok(MetaVars::Column) => {
                let col: TokenStream = {
                    let col = q_span.unwrap().column();

                    parse_quote! {
                        #col
                    }
                };

                Ok(col)
            }
            Ok(MetaVars::Path) => {
                let path = q_span.unwrap().file();

                Ok(parse_quote! {
                    #path
                })
            }
            Ok(MetaVars::AbsPath) => {
                // Cargoプロジェクト以下の場合 local_file / file からは相対パスが返る
                let span_path = q_span.unwrap().local_file().unwrap_or_else(|| {
                    let path = q_span.unwrap().file();
                    PathBuf::from(path)
                });
                let cargo_path =
                    std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| String::new());
                let cargo_path = PathBuf::from(cargo_path);

                let path = if span_path.is_absolute() {
                    span_path.clone()
                } else {
                    cargo_path.join(&span_path)
                };
                let path = path.to_string_lossy();

                Ok(parse_quote! {
                    #path
                })
            }
            Ok(MetaVars::File) => {
                let path = q_span.unwrap().file();
                let file = path.rsplit('/').next().unwrap_or(&path);

                Ok(parse_quote! {
                    #file
                })
            }
            Ok(MetaVars::Expr) => {
                let expr = self.expr;

                Ok(parse_quote! {
                    #expr
                })
            }
            Ok(MetaVars::Count) => {
                let kind = self.kind;
                let count = self.counter().borrow().get_count(kind);
                let val = format!("{count}th {kind}");

                Ok(parse_quote! {
                    #val
                })
            }
            Ok(MetaVars::FnName) => {
                let fn_name = &self
                    .fn_info()
                    .map(|info| info.name())
                    .unwrap_or_else(|| "<unknown>".to_string());

                Ok(parse_quote! {
                    #fn_name
                })
            }
            Ok(MetaVars::FnSig) => {
                let fn_sig = &self
                    .fn_info()
                    .map(|info| info.sig())
                    .unwrap_or_else(|| "<unknown>".to_string());

                Ok(parse_quote! {
                    #fn_sig
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
                                .map(|v| format!(" - ${v}"))
                                .collect::<Vec<_>>()
                                .join("\n"),
                            META_VARS_LIST
                                .iter()
                                .map(|v| format!(" - ${v}"))
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

fn default_method() -> TokenStream {
    // NOTE:
    // $path や $line は eprintln! に直接埋め込みたいところだが、
    // CI側のテストの関係でこのようになっている
    // (恨むならeprintln!の仕様を恨んでください)

    parse_quote! {
        .inspect_err(|e| {
            let path = $path;
            let line = $line;

            ::std::eprintln!("[{path}:L{line}] {e:?}");
        })
    }
}

pub fn method_for_custom() -> TokenStream {
    parse_quote! {
        .hook(|| {
            ::hooq::HooqMeta {
                line: $line,
                column: $column,
                path: $path,
                abs_path: $abs_path,
                file: $file,
                expr: $expr,
                count: $count,
                // TODO: 任意の型の値を任意の変数名に格納した
                // HashMap<String, (TypeId, Box<dyn Any>)> (仮) 型のextra_varsも
                // 参照可能にする
            }
        })
    }
}
