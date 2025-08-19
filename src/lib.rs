use proc_macro::TokenStream;
use syn::{Item, parse_macro_input};

mod impls;

#[proc_macro_attribute]
pub fn hooq(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as Item);

    impls::hooq_impls(item)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
