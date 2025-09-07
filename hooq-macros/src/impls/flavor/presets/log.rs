use proc_macro2::TokenStream;
use syn::parse_quote;

use crate::impls::flavor::Flavor;

pub fn log_flavor() -> Flavor {
    Flavor {
        method: log_method(),
        ..Default::default()
    }
}

// TODO: $expr_str_short を追加
fn log_method() -> TokenStream {
    parse_quote! {
        .inspect_err(|e| {
            let line = $line;
            let expr_str = $expr_str;
            let expr_str = if expr_str.len() > 20 {
                format!("...{}", &expr_str[expr_str.len() - 20..])
            } else {
                expr_str.to_string()
            };

            ::log::error!("(L{line}) {e} from {expr_str}");
        })
    }
}
