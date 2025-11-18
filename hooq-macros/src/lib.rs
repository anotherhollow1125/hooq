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

/// Utility macro to get a pretty string of an expression.
///
/// Example:
///
/// ```rust
/// # use hooq::source_excerpt_helpers::pretty_stringify;
/// # #[rustfmt::skip]
/// # fn main() {
/// let res = pretty_stringify!(
///     @show_line_num,
///     @padding = "bottom"
///     Err([
///         "aaaaaaaaaaaa",
///         "bbbbbbbbbbbb",
///         "cccccccccccc",
///         "dddddddddddd",
///     ])
/// );
///
/// println!("{res}");
/// # }
/// ```
///
/// above will output like this:
///
/// ```plaintext
///    7>    Err([
///    8|        "aaaaaaaaaaaa",
///    9|        "bbbbbbbbbbbb",
///   10|        "cccccccccccc",
///   11|        "dddddddddddd",
///   12|    ])
///     |
/// ```
///
/// It is possible that the indentation of the first line (7>) content is intentionally irregular due to compromise between readability and performance.
///
/// List of options:
///
/// | name | possible values | description |
/// |:----:|:-----|:-----|
/// | `@show_line_num` | (has no field) | Show line numbers |
/// | `@padding` | `top`, `bottom` or `both` (or not specified like `@padding,` then it meens `both` ) | Add padding lines above, below, or both around the snippet (default is not padding) |
///
/// Normally, this macro is designed to be used inside hooq macros to provide better error messages.
///
/// Example: `#[hooq::method(.inspect_err(|_| eprintln!("expr: {}", ::hooq::source_excerpt_helpers::pretty_stringify!($source))))]`
#[proc_macro]
pub fn pretty_stringify(item: TokenStream) -> TokenStream {
    impls::utils::source_excerpt_macros::pretty_stringify(item.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Utility macro to get one line pretty string of an expression.
///
/// Example:
///
/// ```rust
/// # use hooq::source_excerpt_helpers::one_line_stringify;
/// # #[rustfmt::skip]
/// # fn main() {
/// let res = one_line_stringify!(
///     @truncate_str = 4,
///     Err([
///         "aaaaaaaaaaaa",
///         "bbbbbbbbbbbb",
///         "cccccccccccc",
///         "dddddddddddd",
///     ])
/// );
///
/// println!("{res}");
/// # }
/// ```
///
/// above will output like this:
///
/// ```plaintext
/// Err([ "a..a", "b..b", "c..c", "d..d", ])
/// ```
///
/// List of options:
///
/// | name | possible values | description |
/// |:----:|:-----|:-----|
/// | `@truncate_str` | integer (or not specified like `@truncate_str,`) | Truncate strings longer than the specified length (default is 10) |
///
/// Normally, this macro is designed to be used inside hooq macros to provide better error messages.
///
/// Example: `#[hooq::method(.inspect_err(|_| eprintln!("expr: {}", ::hooq::source_excerpt_helpers::one_line_stringify!($source))))]`
#[proc_macro]
pub fn one_line_stringify(item: TokenStream) -> TokenStream {
    impls::utils::source_excerpt_macros::one_line_stringify(item.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Utility macro to get a pretty string of an expression. The difference of this macro and `pretty_stringify!` is that this macro  can omit some lines and truncate long strings.
///
/// Example:
///
/// ```rust
/// # use hooq::source_excerpt_helpers::excerpted_pretty_stringify;
/// # #[rustfmt::skip]
/// # fn main() {
/// let res = excerpted_pretty_stringify!(
///     @excerpt_line = 4,
///     @truncate_str,
///     @show_line_num,
///     @padding,
///     Err([
///         "aaaaaaaaaaaa",
///         "bbbbbbbbbbbb",
///         "cccccccccccc",
///         "dddddddddddd",
///     ])
/// );
///
/// println!("{res}");
/// # }
/// ```
///
/// above will output like this:
///
/// ```plaintext
///     |
///    9>    Err([
/// ...
///   12|        "cccc..cccc",
///   13|        "dddd..dddd",
///   14|    ])
///     |
/// ```
///
/// It is possible that the indentation of the first line (9>) content is intentionally irregular due to compromise between readability and performance.
///
/// List of options:
///
/// | name | possible values | description |
/// |:----:|:-----|:-----|
/// | `@excerpt_line` | integer |  The line number to excerpt from the source code |
/// | `@truncate_str` | integer (or not specified like `@truncate_str,`) | Truncate strings longer than the specified length (default is 10) |
/// | `@show_line_num` | (has no field) | Show line numbers |
/// | `@padding` | `top`, `bottom` or `both` (or not specified like `@padding,` then it meens `both` ) | Add padding lines above, below, or both around the snippet (default is not padding) |
///
/// Normally, this macro is designed to be used inside hooq macros to provide better error messages.
///
/// Example: `#[hooq::method(.inspect_err(|_| eprintln!("expr: {}", ::hooq::source_excerpt_helpers::excerpted_pretty_stringify!($source))))]`
#[proc_macro]
pub fn excerpted_pretty_stringify(item: TokenStream) -> TokenStream {
    impls::utils::source_excerpt_macros::excerpted_pretty_stringify(item.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
