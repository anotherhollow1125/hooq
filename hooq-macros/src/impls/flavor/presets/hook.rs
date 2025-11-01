use proc_macro2::TokenStream;
use syn::parse_quote;

use crate::impls::flavor::Flavor;
use crate::impls::utils::unexpected_error_message::UNEXPECTED_ERROR_MESSAGE;

pub fn hook_flavor() -> Flavor {
    Flavor {
        // NOTE: Traitの存在を前提とするflavorだがユーザーが決定する必要あり
        // trait_uses: Vec::new(), // Default と同じ
        method: hook_method().try_into().expect(UNEXPECTED_ERROR_MESSAGE),
        ..Default::default()
    }
}

fn hook_method() -> TokenStream {
    parse_quote! {
        .hook(|| {
            $hooq_meta
        })
    }
}
