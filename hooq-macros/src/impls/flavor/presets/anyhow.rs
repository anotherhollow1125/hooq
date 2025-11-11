use proc_macro2::TokenStream;
use syn::parse_quote;

use crate::impls::flavor::Flavor;
use crate::impls::utils::unexpected_error_message::UNEXPECTED_ERROR_MESSAGE;

pub fn anyhow_flavor() -> Flavor {
    Flavor {
        trait_uses: vec![parse_quote! { anyhow::Context }],
        method: anyhow_method().try_into().expect(UNEXPECTED_ERROR_MESSAGE),
        ..Default::default()
    }
}

fn anyhow_method() -> TokenStream {
    parse_quote! {
        .with_context(|| {
            let path = $path;
            let line = $line;
            let col = $col;
            let expr = $expr_str_short;

            format!("[{path}:{line}:{col}]
    {expr}")
        })
    }
}
