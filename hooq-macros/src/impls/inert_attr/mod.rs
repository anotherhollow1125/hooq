use std::collections::HashMap;

use proc_macro2::TokenStream;
use syn::{Attribute, Expr, Meta, MetaList, MetaNameValue, Path, parse_quote};

use crate::impls::inert_attr::context::{HookContext, SkipStatus};

pub mod context;
pub mod method;

#[derive(Debug)]
pub struct InertAttrOption {
    pub is_skiped: bool,
    pub is_skiped_all: bool,
    pub method: Option<TokenStream>,
    pub bindings: HashMap<String, Expr>,
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

fn get_binding_name(path: &Path) -> syn::Result<Option<String>> {
    let mut segs = path.segments.iter();

    let Some(maybe_hooq) = segs.next() else {
        return Ok(None);
    };

    if maybe_hooq.ident != "hooq" {
        return Ok(None);
    }

    let maybe_binding = segs.next();

    if segs.next().is_some() {
        return Err(syn::Error::new_spanned(
            path,
            "invalid hooq attribute format. expect: hooq::xxx = ...",
        ));
    }

    Ok(maybe_binding.map(|s| s.ident.to_string()))
}

pub fn extract_hooq_info_from_attrs(attrs: &mut Vec<Attribute>) -> syn::Result<InertAttrOption> {
    let hooq_skip = parse_quote!(hooq::skip);
    let hooq_skip_all = parse_quote!(hooq::skip_all);
    let hooq_method = parse_quote!(hooq::method);
    let hooq_binding = parse_quote!(hooq::binding);
    let hooq_var = parse_quote!(hooq::var);

    let mut is_skiped = false;
    let mut is_skiped_all = false;
    let mut method: Option<TokenStream> = None;
    let mut bindings: HashMap<String, Expr> = HashMap::new();

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
            Meta::List(MetaList { path, tokens, .. })
                if path == &hooq_binding || path == &hooq_var =>
            {
                let MetaNameValue { path, value, .. } =
                    syn::parse2::<MetaNameValue>(tokens.clone())?;

                        "invalid hooq::binding or hooq::var attribute format. expect: hooq::binding(name = value) or hooq::var(name = value)",
                    ));
                };

                bindings.insert(binding.to_string(), value.clone());

                keeps.push(false);
            }
            Meta::NameValue(MetaNameValue { path, value, .. }) => {
                if let Some(binding) = get_binding_name(path)? {
                    bindings.insert(binding, value.clone());
                    keeps.push(false);
                } else {
                    keeps.push(true);
                }
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
        method,
        bindings,
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
