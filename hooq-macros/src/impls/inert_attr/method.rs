use std::path::PathBuf;

use proc_macro2::{Group, Ident, Span, TokenStream, TokenTree};
use syn::parse_quote;

use crate::impls::inert_attr::context::HookInfo;

impl HookInfo<'_> {
    pub fn generate_method(&self, q_span: Span) -> syn::Result<TokenStream> {
        let mut res = TokenStream::new();

        let method = match self.method() {
            Some(method) => method.clone(),
            None => default_method(),
        };
        self.expand_special_vars(method, &mut res, q_span)?;

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
    fn expand_special_vars(
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

                    res.extend([self.special_vars2token_stream(&ident, q_span)?]);
                }
                TokenTree::Group(group) => {
                    if next_is_replace_target {
                        return Err(syn::Error::new(group.span(), "Unexpected token after `$`"));
                    }

                    let mut res_for_group = TokenStream::new();
                    self.expand_special_vars(group.stream(), &mut res_for_group, q_span)?;

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

    fn special_vars2token_stream(
        &self,
        var_ident: &Ident,
        #[allow(unused)] q_span: Span,
    ) -> syn::Result<TokenStream> {
        match var_ident.to_string().as_str() {
            "line" => {
                let line: TokenStream = {
                    let line = q_span.unwrap().line();

                    parse_quote! {
                        #line
                    }
                };

                Ok(line)
            }
            "column" | "col" => {
                let col: TokenStream = {
                    let col = q_span.unwrap().column();

                    parse_quote! {
                        #col
                    }
                };

                Ok(col)
            }
            "path" => {
                let path = q_span.unwrap().file();

                Ok(parse_quote! {
                    #path
                })
            }
            "abspath" | "abs_path" => {
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
            "file" => {
                let path = q_span.unwrap().file();
                let file = path.rsplit('/').next().unwrap_or(&path);

                Ok(parse_quote! {
                    #file
                })
            }
            "expr" => {
                let expr = self.expr;

                Ok(parse_quote! {
                    #expr
                })
            }
            "nth" | "count" => {
                let kind = self.kind;
                let count = self.counter().borrow().get_count(kind);
                let val = format!("{count}th {kind}");

                Ok(parse_quote! {
                    #val
                })
            }
            // TODO: tag 以外の変数も扱えるようにする
            "tag" => {
                let res = if let Some(tag) = self.tag() {
                    parse_quote! {
                        #tag
                    }
                } else {
                    parse_quote! {
                        "(no tag)"
                    }
                };

                Ok(res)
            }
            "fn_name" => {
                let fn_name = &self
                    .fn_info()
                    .map(|info| info.name())
                    .unwrap_or_else(|| "<unknown>".to_string());

                Ok(parse_quote! {
                    #fn_name
                })
            }
            "fn_sig" => {
                let fn_sig = &self
                    .fn_info()
                    .map(|info| info.sig())
                    .unwrap_or_else(|| "<unknown>".to_string());

                Ok(parse_quote! {
                    #fn_sig
                })
            }
            _ => Err(syn::Error::new(
                var_ident.span(),
                format!("Unknown special variable: {var_ident}"),
            )),
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
            ::hooq::HooqInfo {
                line: $line,
                column: $column,
                path: $path,
                abs_path: $abs_path,
                file: $file,
                expr: $expr,
                count: $count,
                // TODO: extra_vars,
            }
        })
    }
}
