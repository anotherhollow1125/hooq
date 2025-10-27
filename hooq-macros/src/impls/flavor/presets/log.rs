use proc_macro2::TokenStream;
use syn::parse_quote;

use crate::impls::flavor::Flavor;

pub fn log_flavor() -> Flavor {
    Flavor {
        method: log_method().into(),
        ..Default::default()
    }
}

// TODO: $expr_str_short を追加
fn log_method() -> TokenStream {
    parse_quote! {
        .inspect_err(|e| {
            let path = $abs_path;
            let line = $line;
            let col = $col;
            let expr = $expr_str_short;

            ::log::error!("({path}:{line}:{col}) {e}
    {expr}");
        })
    }
}
