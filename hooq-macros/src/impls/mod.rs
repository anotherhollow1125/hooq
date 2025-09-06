use proc_macro2::TokenStream;
use quote::quote;
use syn::Item;

use crate::impls::inert_attr::context::HookContext;
use crate::impls::root_attr::{RootAttribute, RootContext};

pub mod flavor;
pub mod inert_attr;
pub mod root_attr;
pub mod utils;
mod walker;

pub fn hooq_impls(root_attr: TokenStream, mut item: Item) -> syn::Result<TokenStream> {
    let root_attr: RootAttribute = syn::parse2(root_attr)?;
    let root_option = RootContext::load(root_attr)?;
    let trait_use_paths = root_option.trait_uses_token_stream();
    let context = HookContext::new(root_option);

    walker::walk_item(&mut item, &context)?;

    Ok(quote! {
        #trait_use_paths

        #item
    })
}
