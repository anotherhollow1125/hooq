use proc_macro2::TokenStream;
use syn::parse_quote;

use crate::impls::flavor::Flavor;
use crate::impls::utils::unexpected_error_message::UNEXPECTED_ERROR_MESSAGE;

pub fn log_flavor() -> Flavor {
    Flavor {
        method: log_method().try_into().expect(UNEXPECTED_ERROR_MESSAGE),
        ..Default::default()
    }
}

// TODO: $expr_str_short を追加
fn log_method() -> TokenStream {
    parse_quote! {
        .inspect_err(|e| {
            let path = $path;
            let line = $line;
            let col = $col;
            let expr = $expr_str_short;

            ::log::error!("({path}:{line}:{col}) {e}
    {expr}");
        })
    }
}
