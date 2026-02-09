use std::collections::HashMap;

use proc_macro2::TokenStream;
use syn::parse_quote;

use crate::impls::flavor::{FlavorNode, FlavorSettingField, FlavorSettings};
use crate::impls::utils::unexpected_error_message::UNEXPECTED_ERROR_MESSAGE;

pub fn anyhow_flavor() -> FlavorNode {
    let settings = FlavorSettings {
        trait_uses: FlavorSettingField::new(vec![parse_quote! { ::anyhow::Context }]),
        method: FlavorSettingField::new(
            anyhow_method().try_into().expect(UNEXPECTED_ERROR_MESSAGE),
        ),
        ..Default::default()
    };

    FlavorNode {
        settings,
        sub_flavors: HashMap::new(),
    }
}

fn anyhow_method() -> TokenStream {
    let excerpted_helpers_path = crate::impls::utils::get_source_excerpt_helpers_name_space();

    parse_quote! {
        .with_context(|| {
            let path = $path;
            let line = $line;
            let col = $col;
            let expr = #excerpted_helpers_path ::excerpted_pretty_stringify!($source);

            format!("[{path}:{line}:{col}]\n{expr}")
        })
    }
}
