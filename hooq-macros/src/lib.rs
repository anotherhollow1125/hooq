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

/// Utility macro to truncate string literals in an expression.
///
/// examples:
/// - `truncate_lit_str!(func_call("0123456789abcdefghijklmnopqrstuvwxyz"))` becomes `func_call("0123..wxyz")`
/// - `truncate_lit_str!(@max_len=36, func_call("0123456789abcdefghijklmnopqrstuvwxyz"))` becomes `func_call("0123456789abcdefghijklmnopqrstuvwxyz")`
#[proc_macro]
pub fn truncate_lit_str(item: TokenStream) -> TokenStream {
    impls::utils::source_excerpt_macros::truncate_lit_str(item.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Utility macro to get pretty source excerpt of an expression.
///
/// examples:
/// TODO
#[proc_macro]
pub fn pretty_stringify(item: TokenStream) -> TokenStream {
    impls::utils::source_excerpt_macros::pretty_stringify(item.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Utility macro to get one line source excerpt of an expression.
///
/// examples:
/// TODO
#[proc_macro]
pub fn one_line_stringify(item: TokenStream) -> TokenStream {
    impls::utils::source_excerpt_macros::one_line_stringify(item.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Utility macro to get source excerpt of an expression.
///
/// examples:
/// TODO
#[proc_macro]
pub fn excerpted_pretty_stringify(item: TokenStream) -> TokenStream {
    impls::utils::source_excerpt_macros::excerpted_pretty_stringify(item.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
