use std::collections::HashMap;

use proc_macro2::TokenStream;

use crate::impls::flavor::Flavor;
use crate::impls::inert_attr::context::HookTargetSwitch;

pub fn empty_flavor() -> Flavor {
    Flavor {
        trait_uses: Vec::new(),
        method: TokenStream::new(),
        hook_targets: HookTargetSwitch {
            question: false,
            return_: false,
            tail_expr: false,
        },
        tail_expr_idents: Vec::new(),
        ignore_tail_expr_idents: Vec::new(),
        result_types: Vec::new(),
        hook_in_macros: false,
        bindings: HashMap::new(),
        sub_flavors: HashMap::new(),
    }
}
