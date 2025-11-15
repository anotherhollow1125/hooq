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

fn log_method() -> TokenStream {
    let excerpted_helpers_path = crate::impls::utils::get_source_excerpt_helpers_name_space();

    parse_quote! {
        .inspect_err(|e| {
            let path = $path;
            let line = $line;
            let col = $col;
            let expr = #excerpted_helpers_path ::excerpted_pretty_stringify!($source);

            ::log::error!("({path}:{line}:{col}) {e}\n{expr}");
        })
    }
}
