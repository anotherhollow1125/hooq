use proc_macro2::TokenStream;
use syn::{Attribute, Meta, MetaList, parse::Parse, parse_quote};

use crate::impls::utils::strip_attr;

pub struct InertAttrOption {
    pub is_skiped: bool,
    pub tag: Option<String>,
    pub method: Option<TokenStream>,
}

pub fn extract_hooq_info_from_attrs(attrs: &mut [Attribute]) -> syn::Result<InertAttrOption> {
    let hooq_skip = parse_quote!(hooq::skip);
    let hooq_tag = parse_quote!(hooq::tag);
    let hooq_method = parse_quote!(hooq::method);

    let mut is_skiped = false;
    let mut tag: Option<String> = None;
    let mut method: Option<TokenStream> = None;

    for attr in attrs.iter_mut() {
        match &attr.meta {
            Meta::Path(p) if p == &hooq_skip => {
                strip_attr(attr);
                is_skiped = true;
            }
            Meta::List(MetaList { path, tokens, .. }) if path == &hooq_method => {
                method = Some(tokens.clone());
                strip_attr(attr);
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
                strip_attr(attr);
            }
            _ => {}
        }
    }

    Ok(InertAttrOption {
        is_skiped,
        tag,
        method,
    })
}
