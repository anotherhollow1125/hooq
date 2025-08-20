use proc_macro2::TokenStream;
use quote::quote;
use syn::Item;

use crate::impls::attr::context::HookContext;

pub mod attr;
pub mod utils;
mod walker;

pub fn hooq_impls(mut item: Item) -> syn::Result<TokenStream> {
    let context = HookContext::new();

    walker::walk_item(&mut item, &context)?;

    Ok(quote! {
        #item
    })
}
