use proc_macro2::{Group, Ident, Span, TokenStream, TokenTree};
use syn::{Attribute, Expr, Meta, MetaList, Path, Token, parse::Parse, parse_quote};

use crate::impls::utils::strip_attr;

#[derive(Debug)]
pub struct HooqOption {
    pub use_: Vec<Path>,
    pub method: TokenStream,
}

fn default_method() -> TokenStream {
    parse_quote! {
        .map_err(|e| {
            ::log::error!("{:?} @ file: {}, line: {}", e, $file, $line);
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
    pub fn new_from_attrs(attrs: &mut [Attribute]) -> Result<HooqOption, syn::Error> {
        let mut option = HooqOption::default();

        for attr in attrs.iter_mut() {
            if let Some(paths) = pickup_use(attr)? {
                option.use_.extend(paths);
                strip_attr(attr);
                continue;
            }

            if let Some(method) = pickup_method(attr) {
                option.method = method;
                strip_attr(attr);
                continue;
            }
        }

        Ok(option)
    }

    pub fn generate_method(&self, q_span: Span, context: &ReplaceContext) -> TokenStream {
        let mut res = TokenStream::new();
        expand_special_vars(self.method.clone(), &mut res, q_span, context);
        res
    }
}

#[derive(Clone, Copy, Debug)]
pub struct FunctionInfo<'a> {
    pub pos: usize,
    pub name: &'a str,
    pub sig: &'a str,
    pub path: &'a str,
}

#[derive(Clone, Copy, Debug)]
pub enum ReplaceKind {
    Question,
    Return,
}

#[derive(Clone, Copy, Debug)]
pub struct ReplaceContext<'a> {
    pub expr: &'a Expr,
    pub count: usize,
    pub kind: ReplaceKind,
    pub tag: Option<&'a str>,
    pub info: &'a FunctionInfo<'a>,
}

fn expand_special_vars(
    ts: TokenStream,
    res: &mut TokenStream,
    q_span: Span,
    context: &ReplaceContext,
) {
    let mut next_is_replace_target = false;
    for tt in ts.into_iter() {
        match tt {
            TokenTree::Punct(punct) => {
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

                res.extend([special_vars2token_stream(&ident, q_span)]);
            }
            TokenTree::Group(group) => {
                let mut res_for_group = TokenStream::new();
                expand_special_vars(group.stream(), &mut res_for_group, q_span, context);

                let new_group = Group::new(group.delimiter(), res_for_group);

                res.extend([TokenTree::Group(new_group)]);
            }
            TokenTree::Literal(literal) => {
                res.extend([TokenTree::Literal(literal)]);
            }
        }
    }
}

fn special_vars2token_stream(var_ident: &Ident, #[allow(unused)] q_span: Span) -> TokenStream {
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
                    line!()
                }
            };

            line
        }
        "file" => {
            parse_quote! {
                file!()
            }
        }
        _ => parse_quote!(#var_ident),
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
