use std::collections::HashMap;

use syn::{Attribute, Expr};

use crate::impls::inert_attr::context::{HookContext, HookTargetSwitch, SkipStatus};
use crate::impls::method::Method;

pub mod context;
mod parse;

#[derive(Debug)]
pub struct InertAttribute {
    pub method: Option<Method>,
    pub hook_targets: Option<HookTargetSwitch>,
    pub tail_expr_idents: Option<Vec<String>>,
    pub ignore_tail_expr_idents: Option<Vec<String>>,
    pub result_types: Option<Vec<String>>,
    pub hook_in_macros: Option<bool>,
    pub bindings: HashMap<String, Expr>,
    pub is_skipped: bool,
    pub is_skipped_all: bool,
}

impl InertAttribute {
    pub fn get_skip_status(&self) -> Option<SkipStatus> {
        match (self.is_skipped_all, self.is_skipped) {
            (true, _) => Some(SkipStatus::SkipAll),
            (false, true) => Some(SkipStatus::SkipSameScope),
            _ => None,
        }
    }
}

pub struct HandleInertAttrsResult {
    pub is_skipped: bool,
    pub new_context: HookContext,
}

pub fn handle_inert_attrs(
    attrs: &mut Vec<Attribute>,
    context: &HookContext,
) -> syn::Result<HandleInertAttrsResult> {
    let option = parse::extract_hooq_info_from_attrs(attrs)?;

    Ok(HandleInertAttrsResult {
        is_skipped: option.is_skipped || option.is_skipped_all || context.is_skipped(),
        new_context: HookContext::updated_by_inert_attr(context, option),
    })
}
