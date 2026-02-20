use std::collections::HashMap;

use proc_macro2::TokenStream;
use syn::parse_quote;

use crate::impls::flavor::{FlavorNode, FlavorSettingField, FlavorSettings};
use crate::impls::inert_attr::context::HookTargetSwitch;
use crate::impls::utils::unexpected_error_message::UNEXPECTED_ERROR_MESSAGE;

pub fn empty_flavor() -> FlavorNode {
    let settings = FlavorSettings {
        trait_uses: FlavorSettingField::new(Vec::new()),
        method: FlavorSettingField::new(empty_method().try_into().expect(UNEXPECTED_ERROR_MESSAGE)),
        hook_targets: FlavorSettingField::new(HookTargetSwitch {
            question: false,
            return_: false,
            tail_expr: false,
        }),
        tail_expr_idents: FlavorSettingField::new(Vec::new()),
        ignore_tail_expr_idents: FlavorSettingField::new(Vec::new()),
        result_types: FlavorSettingField::new(Vec::new()),
        hook_in_macros: FlavorSettingField::new(false),
        bindings: FlavorSettingField::new(HashMap::new()),
    };

    FlavorNode {
        settings,
        sub_flavors: HashMap::new(),
    }
}

fn empty_method() -> TokenStream {
    parse_quote! {
        $expr
    }
}
