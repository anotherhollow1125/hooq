use std::collections::HashMap;

use proc_macro2::TokenStream;
use syn::parse_quote;

use crate::impls::flavor::{FlavorNode, FlavorSettingField, FlavorSettings};
use crate::impls::utils::unexpected_error_message::UNEXPECTED_ERROR_MESSAGE;

pub fn hook_flavor() -> FlavorNode {
    let settings = FlavorSettings {
        // NOTE: Traitの存在を前提とするflavorだがユーザーが決定する必要あり
        // trait_uses: FlavorSettingField::new(Vec::new()), // Default と同じ
        method: FlavorSettingField::new(hook_method().try_into().expect(UNEXPECTED_ERROR_MESSAGE)),
        ..Default::default()
    };

    FlavorNode {
        settings,
        sub_flavors: HashMap::new(),
    }
}

fn hook_method() -> TokenStream {
    parse_quote! {
        .hook(|| {
            $hooq_meta
        })
    }
}
