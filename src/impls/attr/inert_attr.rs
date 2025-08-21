use proc_macro2::TokenStream;
use syn::parse::Parse;
use syn::{Attribute, Meta, MetaList, parse_quote};

use crate::impls::attr::context::{HookContext, SkipStatus};

#[derive(Debug)]
pub struct InertAttrOption {
    pub is_skiped: bool,
    pub is_skiped_all: bool,
    pub tag: Option<String>,
    pub method: Option<TokenStream>,
}

impl InertAttrOption {
    pub fn get_skip_status(&self) -> Option<SkipStatus> {
        match (self.is_skiped_all, self.is_skiped) {
            (true, _) => Some(SkipStatus::SkipAll),
            (false, true) => Some(SkipStatus::SkipSameScope),
            _ => None,
        }
    }
}

pub fn extract_hooq_info_from_attrs(attrs: &mut Vec<Attribute>) -> syn::Result<InertAttrOption> {
    let hooq_skip = parse_quote!(hooq::skip);
    let hooq_skip_all = parse_quote!(hooq::skip_all);
    let hooq_tag = parse_quote!(hooq::tag);
    let hooq_method = parse_quote!(hooq::method);

    let mut is_skiped = false;
    let mut is_skiped_all = false;
    let mut tag: Option<String> = None;
    let mut method: Option<TokenStream> = None;

    let mut keeps = Vec::with_capacity(attrs.len());
    for attr in attrs.iter_mut() {
        match &attr.meta {
            Meta::Path(p) if p == &hooq_skip => {
                is_skiped = true;
                keeps.push(false);
            }
            Meta::Path(p) if p == &hooq_skip_all => {
                is_skiped_all = true;
                keeps.push(false);
            }
            Meta::List(MetaList { path, tokens, .. }) if path == &hooq_method => {
                method = Some(tokens.clone());
                keeps.push(false);
            }
            Meta::List(MetaList { path, tokens, .. }) if path == &hooq_tag => {
                struct Tag(String);

                impl Parse for Tag {
                    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
                        let tag = input.parse::<syn::LitStr>()?;
                        Ok(Self(tag.value()))
                    }
                }

                let t = syn::parse2::<Tag>(tokens.clone())?;
                tag = Some(t.0);
                keeps.push(false);
            }
            /*
            // ルート部分でのパースで再利用するので一旦コメントアウトで
            // TODO: 削除
            Meta::List(MetaList { path, tokens, .. }) if path == &hooq_trait_use => {
                struct Paths(Vec<Path>);

                impl Parse for Paths {
                    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
                        let paths = input.parse_terminated(Path::parse, Token![,])?;
                        Ok(Self(paths.into_iter().collect()))
                    }
                }

                let paths = syn::parse2::<Paths>(tokens.clone())?.0;

                trait_use.extend(paths);
                keeps.push(false);
            }
            */
            _ => {
                keeps.push(true);
            }
        }
    }

    // ref: https://doc.rust-lang.org/alloc/vec/struct.Vec.html#method.retain
    let mut keeps_iter = keeps.iter();
    attrs.retain(|_| *keeps_iter.next().unwrap());

    Ok(InertAttrOption {
        is_skiped,
        is_skiped_all,
        tag,
        method,
    })
}

pub struct HandleInertAttrsResult<'a> {
    pub is_skiped: bool,
    pub new_context: HookContext<'a>,
}

pub fn handle_inert_attrs<'a>(
    attrs: &mut Vec<Attribute>,
    context: &'a HookContext,
) -> syn::Result<HandleInertAttrsResult<'a>> {
    let option = extract_hooq_info_from_attrs(attrs)?;

    Ok(HandleInertAttrsResult {
        is_skiped: option.is_skiped || option.is_skiped_all || context.is_skiped(),
        new_context: HookContext::updated_by_inert_attr(context, option),
    })
}
