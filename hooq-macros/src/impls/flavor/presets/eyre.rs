use proc_macro2::TokenStream;
use syn::parse_quote;

use crate::impls::flavor::Flavor;
use crate::impls::utils::unexpected_error_message::UNEXPECTED_ERROR_MESSAGE;

pub fn eyre_flavor() -> Flavor {
    Flavor {
        trait_uses: vec![
            parse_quote! { ::eyre::ContextCompat },
            parse_quote! { ::eyre::WrapErr },
        ],
        method: eyre_method().try_into().expect(UNEXPECTED_ERROR_MESSAGE),
        ..Default::default()
    }
}

pub fn color_eyre_flavor() -> Flavor {
    Flavor {
        trait_uses: vec![
            parse_quote! { ::color_eyre::eyre::ContextCompat },
            parse_quote! { ::color_eyre::eyre::WrapErr },
        ],
        method: eyre_method().try_into().expect(UNEXPECTED_ERROR_MESSAGE),
        ..Default::default()
    }
}

fn eyre_method() -> TokenStream {
    let excerpted_helpers_path = crate::impls::utils::get_source_excerpt_helpers_name_space();

    parse_quote! {
        .wrap_err_with(|| {
            let path = $path;
            let line = $line;
            let col = $col;
            let expr = #excerpted_helpers_path ::excerpted_pretty_stringify!($source);

            format!("[{path}:{line}:{col}]\n{expr}")
        })
    }
}
