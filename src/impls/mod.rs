use proc_macro2::TokenStream;
use quote::quote;
use syn::Item;

use crate::impls::inert_attr::context::HookContext;
use crate::impls::root_attr::HooqRootOption;

pub mod inert_attr;
pub mod root_attr;
pub mod utils;
mod walker;

pub fn hooq_impls(root_attr: TokenStream, mut item: Item) -> syn::Result<TokenStream> {
    let root_option: HooqRootOption = syn::parse2(root_attr)?;
    let context = HookContext::new();

    walker::walk_item(&mut item, &context)?;

    let paths = root_option.trait_uses_token_stream();

    Ok(quote! {
        #paths

        #item
    })
}
