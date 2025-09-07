use proc_macro::TokenStream;
use syn::{Item, parse_macro_input};

mod impls;

#[proc_macro_attribute]
pub fn hooq(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as Item);

    impls::hooq_impls(attr.into(), item)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro]
pub fn toml_load(input: TokenStream) -> TokenStream {
    impls::flavor::r#macro::toml_load(input.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
