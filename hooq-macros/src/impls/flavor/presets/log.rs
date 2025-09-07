use proc_macro2::TokenStream;
use syn::parse_quote;

use crate::impls::flavor::Flavor;

pub fn log_flavor() -> Flavor {
    Flavor {
        method: log_method(),
        ..Default::default()
    }
}

fn log_method() -> TokenStream {
    parse_quote! {
        .inspect_err(|e| {
            let line = $line;

            ::log::error!("(L{line}) {e}");
        })
    }
}
