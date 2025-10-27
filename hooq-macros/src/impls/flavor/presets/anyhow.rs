use proc_macro2::TokenStream;
use syn::parse_quote;

use crate::impls::flavor::Flavor;

pub fn anyhow_flavor() -> Flavor {
    Flavor {
        trait_uses: vec![parse_quote! { anyhow::Context }],
        method: anyhow_method().into(),
        ..Default::default()
    }
}

// TODO: $expr_str_short を追加
fn anyhow_method() -> TokenStream {
    parse_quote! {
        .with_context(|| {
            let path = $abs_path;
            let line = $line;
            let col = $col;
            let expr = $expr_str_short;

            format!("[{path}:{line}:{col}]
    {expr}")
        })
    }
}
