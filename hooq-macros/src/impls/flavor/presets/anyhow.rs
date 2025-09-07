use proc_macro2::TokenStream;
use syn::parse_quote;

use crate::impls::flavor::Flavor;

pub fn anyhow_flavor() -> Flavor {
    Flavor {
        trait_uses: vec![parse_quote! { anyhow::Context }],
        method: anyhow_method(),
        ..Default::default()
    }
}

// TODO: $expr_str_short を追加
fn anyhow_method() -> TokenStream {
    parse_quote! {
        .with_context(|| {
            let file = $file;
            let line = $line;
            let expr_str = $expr_str;
            let expr_str = if expr_str.len() > 20 {
                format!("...{}", &expr_str[expr_str.len() - 20..])
            } else {
                expr_str.to_string()
            };

            format!("[{file}:L{line}] {expr_str}")
        })
    }
}
