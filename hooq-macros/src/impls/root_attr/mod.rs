use proc_macro2::TokenStream;
use syn::Path;

mod parse;

#[derive(Debug)]
pub struct HooqRootOption {
    pub trait_uses: Vec<Path>,
}

impl HooqRootOption {
    pub fn trait_uses_token_stream(&self) -> TokenStream {
        self.trait_uses
            .iter()
            .map(|p| quote::quote! { use #p as _; })
            .collect()
    }
}
