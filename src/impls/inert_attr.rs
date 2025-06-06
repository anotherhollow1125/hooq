use proc_macro2::TokenStream;
use syn::{Attribute, Meta, MetaList, parse::Parse, parse_quote};

pub struct InertAttrOption {
    pub is_skiped: bool,
    pub is_skiped_all: bool,
    pub tag: Option<String>,
    pub method: Option<TokenStream>,
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
