use std::collections::HashMap;

use proc_macro2::TokenStream;
use syn::parse::Parse;
use syn::{
    Attribute, Expr, LitBool, LitStr, Meta, MetaList, MetaNameValue, Path, Token, parse_quote,
};

use crate::impls::inert_attr::context::{HookContext, HookTargetSwitch, SkipStatus};

pub mod context;
pub mod method;

#[derive(Debug)]
pub struct InertAttribute {
    pub method: Option<TokenStream>,
    pub hook_targets: Option<HookTargetSwitch>,
    pub tail_expr_idents: Option<Vec<String>>,
    pub result_types: Option<Vec<String>>,
    pub hook_in_macros: Option<bool>,
    pub bindings: HashMap<String, Expr>,
    pub is_skiped: bool,
    pub is_skiped_all: bool,
}

impl InertAttribute {
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
            "invalid hooq attribute format. expected: hooq::xxx = ...",
        ));
    }

    Ok(maybe_binding.map(|s| s.ident.to_string()))
}

struct Strings(Vec<String>);

impl Parse for Strings {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Strings(
            input
                .parse_terminated(<LitStr as Parse>::parse, Token![,])?
                .into_iter()
                .map(|lit| lit.value())
                .collect::<Vec<_>>(),
        ))
    }
}

impl TryFrom<Strings> for HookTargetSwitch {
    type Error = String;

    fn try_from(value: Strings) -> Result<Self, Self::Error> {
        let mut switch = HookTargetSwitch {
            question: false,
            return_: false,
            tail_expr: false,
        };

        for s in value.0.iter() {
            match s.as_str() {
                "?" | "question" => switch.question = true,
                "return" => switch.return_ = true,
                "tail_expr" | "tail expr" | "tailexpr" | "tailExpr" => switch.tail_expr = true,
                "all" => {
                    switch.question = true;
                    switch.return_ = true;
                    switch.tail_expr = true;
                }
                e => return Err(e.to_string()),
            }
        }

        Ok(switch)
    }
}

pub fn extract_hooq_info_from_attrs(attrs: &mut Vec<Attribute>) -> syn::Result<InertAttribute> {
    // #[hooq::method(...)]
    let hooq_method = parse_quote!(hooq::method);
    // #[hooq::hook_targets("return", "?", ...)]
    let hooq_hook_targets = parse_quote!(hooq::hook_targets);
    // #[hooq::tail_expr_idents("Ok", ...)]
    let hooq_tail_expr_idents = parse_quote!(hooq::tail_expr_idents);
    // #[hooq::result_types("XxxResult", ...)]
    let hooq_result_types = parse_quote!(hooq::result_types);
    // #[hooq::hook_in_macros(true | false)]
    let hooq_hook_in_macros = parse_quote!(hooq::hook_in_macros);
    // #[hooq::binding(xxx = ...)]
    let hooq_binding = parse_quote!(hooq::binding);
    // #[hooq::var(xxx = ...)]
    let hooq_var = parse_quote!(hooq::var);
    // #[hooq::skip]
    let hooq_skip = parse_quote!(hooq::skip);
    // #[hooq::skip_all]
    let hooq_skip_all = parse_quote!(hooq::skip_all);

    let mut method: Option<TokenStream> = None;
    let mut hook_targets: Option<HookTargetSwitch> = None;
    let mut tail_expr_idents: Option<Vec<String>> = None;
    let mut result_types: Option<Vec<String>> = None;
    let mut hook_in_macros: Option<bool> = None;
    let mut bindings: HashMap<String, Expr> = HashMap::new();
    let mut is_skiped = false;
    let mut is_skiped_all = false;

    let mut keeps = Vec::with_capacity(attrs.len());
    for attr in attrs.iter_mut() {
        match &attr.meta {
            // method
            Meta::List(MetaList { path, tokens, .. }) if path == &hooq_method => {
                method = Some(tokens.clone());
                keeps.push(false);
            }

            // hook_targets
            Meta::List(MetaList { path, tokens, .. }) if path == &hooq_hook_targets => {
                let strings = syn::parse2::<Strings>(tokens.clone())?;
                match HookTargetSwitch::try_from(strings) {
                    Ok(switch) => {
                        hook_targets = Some(switch);
                        keeps.push(false);
                    }
                    Err(e) => {
                        return Err(syn::Error::new_spanned(
                            path,
                            format!(
                                r#"invalid hooq::hook_targets attribute value. got: {e}
expected: "?", "return", "tail_expr""#,
                            ),
                        ));
                    }
                }
            }

            // tail_expr_idents
            Meta::List(MetaList { path, tokens, .. }) if path == &hooq_tail_expr_idents => {
                let strings = syn::parse2::<Strings>(tokens.clone())?;
                tail_expr_idents = Some(strings.0);
                keeps.push(false);
            }

            // result_types
            Meta::List(MetaList { path, tokens, .. }) if path == &hooq_result_types => {
                let strings = syn::parse2::<Strings>(tokens.clone())?;
                result_types = Some(strings.0);
                keeps.push(false);
            }

            // hook_in_macros
            Meta::List(MetaList { path, tokens, .. }) if path == &hooq_hook_in_macros => {
                let hook = syn::parse2::<LitBool>(tokens.clone())?;
                hook_in_macros = Some(hook.value());
                keeps.push(false);
            }

            // is_skiped
            Meta::Path(p) if p == &hooq_skip => {
                is_skiped = true;
                keeps.push(false);
            }
            // is_skiped_all
            Meta::Path(p) if p == &hooq_skip_all => {
                is_skiped_all = true;
                keeps.push(false);
            }
            // 残りは bindings
            Meta::List(MetaList { path, tokens, .. })
                if path == &hooq_binding || path == &hooq_var =>
            {
                let MetaNameValue { path, value, .. } =
                    syn::parse2::<MetaNameValue>(tokens.clone())?;

                let Some(binding) = path.get_ident() else {
                    return Err(syn::Error::new_spanned(
                        path,
                        "invalid hooq::binding or hooq::var attribute format. expected: hooq::binding(name = value) or hooq::var(name = value)",
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

    Ok(InertAttribute {
        method,
        hook_targets,
        tail_expr_idents,
        result_types,
        hook_in_macros,
        bindings,
        is_skiped,
        is_skiped_all,
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
