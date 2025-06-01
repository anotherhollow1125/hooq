use proc_macro2::{Group, Ident, Span, TokenStream, TokenTree};
use syn::{Attribute, Meta, MetaList, Path, Token, parse::Parse, parse_quote};

pub mod context;
use context::ReplaceContext;

#[derive(Debug)]
pub struct HooqOption {
    pub use_: Vec<Path>,
    pub method: TokenStream,
}

fn default_method() -> TokenStream {
    #[cfg(feature = "nightly")]
    parse_quote! {
        .map_err(|e| {
            ::log::error!("{:?} @ file: {}, line: {}", e, $file, $line);
            e
        })
    }

    #[cfg(not(feature = "nightly"))]
    parse_quote! {
        .map_err(|e| {
            ::log::error!("{:?} @ file: {}", e, file!());
            e
        })
    }
}

impl Default for HooqOption {
    fn default() -> Self {
        Self {
            use_: vec![],
            method: default_method(),
        }
    }
}

impl HooqOption {
    pub fn new_from_attrs(attrs: &mut Vec<Attribute>) -> Result<HooqOption, syn::Error> {
        let mut option = HooqOption::default();

        let mut keeps = Vec::with_capacity(attrs.len());
        for attr in attrs.iter_mut() {
            if let Some(paths) = pickup_use(attr)? {
                option.use_.extend(paths);
                keeps.push(false);
                continue;
            }

            if let Some(method) = pickup_method(attr) {
                option.method = method;
                keeps.push(false);
                continue;
            }

            keeps.push(true);
        }

        // ref: https://doc.rust-lang.org/alloc/vec/struct.Vec.html#method.retain
        let mut keeps_iter = keeps.iter();
        attrs.retain(|_| *keeps_iter.next().unwrap());

        Ok(option)
    }

    pub fn generate_method(
        &self,
        q_span: Span,
        context: &ReplaceContext,
    ) -> syn::Result<TokenStream> {
        let mut res = TokenStream::new();

        let method = match context.override_method() {
            Some(override_method) => override_method.clone(),
            None => self.method.clone(),
        };
        expand_special_vars(method, &mut res, q_span, context)?;
        Ok(res)
    }
}

fn expand_special_vars(
    ts: TokenStream,
    res: &mut TokenStream,
    q_span: Span,
    context: &ReplaceContext,
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

                res.extend([special_vars2token_stream(&ident, q_span, context)?]);
            }
            TokenTree::Group(group) => {
                if next_is_replace_target {
                    return Err(syn::Error::new(group.span(), "Unexpected token after `$`"));
                }

                let mut res_for_group = TokenStream::new();
                expand_special_vars(group.stream(), &mut res_for_group, q_span, context)?;

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
    var_ident: &Ident,
    #[allow(unused)] q_span: Span,
    context: &ReplaceContext,
) -> syn::Result<TokenStream> {
    match var_ident.to_string().as_str() {
        "line" => {
            #[cfg(feature = "nightly")]
            let line: TokenStream = {
                let line = q_span.start().line;

                parse_quote! {
                    #line
                }
            };

            #[cfg(not(feature = "nightly"))]
            let line: TokenStream = {
                parse_quote! {
                    {
                        #[deprecated(note = "`$line` requires the `nightly` feature to return the precise line number.")]
                        const NIGHTLY_NEEDS: () = ();

                        let _ = NIGHTLY_NEEDS;

                        line!()
                    }
                }
            };

            Ok(line)
        }
        "file" => Ok(parse_quote! {
            file!()
        }),
        "expr" => {
            let expr = context.expr.clone();

            Ok(parse_quote! {
                #expr
            })
        }
        "nth" | "count" => {
            let kind = context.kind;
            let count = context.counter().borrow().get_count(kind);
            let val = format!("{}th {}", count, kind);

            Ok(parse_quote! {
                #val
            })
        }
        "tag" => {
            let res = if let Some(tag) = context.tag() {
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
            let fn_name = &context.fn_info().name;

            Ok(parse_quote! {
                #fn_name
            })
        }
        "fn_sig" => {
            let fn_sig = &context.fn_info().sig;

            Ok(parse_quote! {
                #fn_sig
            })
        }
        _ => Err(syn::Error::new(
            var_ident.span(),
            format!("Unknown special variable: {}", var_ident),
        )),
    }
}

fn pickup_use(attr: &Attribute) -> Result<Option<Vec<Path>>, syn::Error> {
    let Meta::List(MetaList { path, tokens, .. }) = &attr.meta else {
        return Ok(None);
    };

    let hooq_use = parse_quote!(hooq::use_);

    if path != &hooq_use {
        return Ok(None);
    }

    struct Paths(Vec<Path>);

    impl Parse for Paths {
        fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
            let paths = input.parse_terminated(Path::parse, Token![,])?;
            Ok(Self(paths.into_iter().collect()))
        }
    }

    let paths = syn::parse2::<Paths>(tokens.clone())?.0;

    Ok(Some(paths))
}

fn pickup_method(attr: &Attribute) -> Option<TokenStream> {
    let Meta::List(MetaList { path, tokens, .. }) = &attr.meta else {
        return None;
    };

    let hooq_method = parse_quote!(hooq::method);

    if path != &hooq_method {
        return None;
    }

    Some(tokens.clone())
}
