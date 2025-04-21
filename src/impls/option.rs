use proc_macro2::{Group, Ident, Span, TokenStream, TokenTree};
use quote::ToTokens;
use syn::{Attribute, Meta, MetaList, Path, Token, parse::Parse, parse_quote};

use crate::impls::utils::strip_attr;

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

    pub fn generate_method(
        &self,
        q_span: Span,
        context: &ReplaceContext,
    ) -> syn::Result<TokenStream> {
        let mut res = TokenStream::new();
        expand_special_vars(self.method.clone(), &mut res, q_span, context)?;
        Ok(res)
    }
}

#[derive(Clone, Debug)]
pub struct FunctionInfo {
    pub name: String,
    pub sig: String,
}

pub trait ExtractFunctionInfo {
    fn extract_function_info(&self) -> syn::Result<FunctionInfo>;
}

impl ExtractFunctionInfo for syn::ItemFn {
    fn extract_function_info(&self) -> syn::Result<FunctionInfo> {
        let sig = self.sig.to_token_stream().to_string();
        let name = self.sig.ident.to_string();

        Ok(FunctionInfo { name, sig })
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ReplaceKind {
    Question,
    Return,
    TailExpr,
}

impl std::fmt::Display for ReplaceKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReplaceKind::Question => write!(f, "?"),
            ReplaceKind::Return => write!(f, "return"),
            ReplaceKind::TailExpr => write!(f, "tail expr"),
        }
    }
}

#[derive(Debug)]
pub struct Counter {
    question: usize,
    return_: usize,
    tail_expr: usize,
}

impl Counter {
    pub fn new() -> Self {
        Self {
            question: 0,
            return_: 0,
            tail_expr: 0,
        }
    }

    pub fn count_up(&mut self, kind: ReplaceKind) {
        match kind {
            ReplaceKind::Question => self.question += 1,
            ReplaceKind::Return => self.return_ += 1,
            ReplaceKind::TailExpr => self.tail_expr += 1,
        }
    }

    pub fn get_count(&self, kind: ReplaceKind) -> usize {
        match kind {
            ReplaceKind::Question => self.question,
            ReplaceKind::Return => self.return_,
            ReplaceKind::TailExpr => self.tail_expr,
        }
    }
}

#[derive(Debug)]
pub struct ReplaceContext<'a> {
    pub expr: String,
    pub kind: ReplaceKind,
    pub tag: Option<String>,

    pub counter: &'a mut Counter,
    pub fn_info: &'a FunctionInfo,
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
            let count = context.counter.get_count(kind);
            let val = format!("{}th {}", count, kind);

            Ok(parse_quote! {
                #val
            })
        }
        "tag" => {
            let res = if let Some(tag) = &context.tag {
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
            let fn_name = &context.fn_info.name;

            Ok(parse_quote! {
                #fn_name
            })
        }
        "fn_sig" => {
            let fn_sig = &context.fn_info.sig;

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
