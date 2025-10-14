#![doc = include_str!("../docs/README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/anotherhollow1125/hooq/refs/heads/main/assets/hooq_logo.svg"
)]
#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/anotherhollow1125/hooq/refs/heads/main/assets/hooq_logo.svg"
)]

use proc_macro::TokenStream;
use syn::{Item, parse_macro_input};

mod impls;

/// hooq macro attribute. Methods are hooked to ? etc. in the function of the item with this macro.
#[proc_macro_attribute]
pub fn hooq(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as Item);

    impls::hooq_impls(attr.into(), item)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
