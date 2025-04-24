use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemFn;

pub mod option;
pub mod utils;
mod walker;

use crate::impls::option::ExtractFunctionInfo;
pub use option::HooqOption;

pub fn hooq_impls(mut f: ItemFn) -> syn::Result<TokenStream> {
    let hooq_option = HooqOption::new_from_attrs(&mut f.attrs)?;

    let fn_info = f.extract_function_info()?;
    let mut context = option::PartialReplaceContext::new(fn_info);
    let stmts_len = f.block.stmts.len();
    f.block
        .stmts
        .iter_mut()
        .enumerate()
        .map(|(i, stmt)| {
            walker::walk_stmt(stmt, true, i == stmts_len - 1, &hooq_option, &mut context)
        })
        .collect::<syn::Result<Vec<()>>>()?;

    Ok(quote! {
        #f
    })
}
