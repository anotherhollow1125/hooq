use proc_macro::TokenStream;
use syn::{ItemFn, parse_macro_input};

mod impls;

#[proc_macro_attribute]
pub fn hooq(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let f = parse_macro_input!(item as ItemFn);

    impls::hooq_impls(f)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
