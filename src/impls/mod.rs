use proc_macro2::TokenStream;
use quote::quote;
use syn::Item;
use syn::spanned::Spanned;

use crate::impls::attr::context::HookContext;
use crate::impls::attr::inert_attr::extract_hooq_info_from_attrs;
use crate::impls::utils::get_attrs_from_item;

pub mod attr;
pub mod utils;
mod walker;

pub fn hooq_impls(mut item: Item) -> syn::Result<TokenStream> {
    let span = item.span();
    let attrs = get_attrs_from_item(&mut item).ok_or_else(move || {
        syn::Error::new(
            span,
            "Failed to extract attributes. (Item::Verbatim is not supported.)",
        )
    })?;
    let inert_attr_option = extract_hooq_info_from_attrs(attrs)?;
    let context = HookContext::init(inert_attr_option);

    walker::walk_item(&mut item, &context)?;

    Ok(quote! {
        #item
    })
}
