use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemFn;
use utils::return_type_is_result;

mod inert_attr;
pub mod option;
pub mod utils;
mod walker;

use crate::impls::option::context::ExtractFunctionInfo;
use crate::impls::option::context::PartialReplaceContext;
pub use option::HooqOption;

pub fn hooq_impls(mut f: ItemFn) -> syn::Result<TokenStream> {
    let hooq_option = HooqOption::new_from_attrs(&mut f.attrs)?;

    let fn_info = f.extract_function_info()?;
    // TODO: 関数についているtag (や今後の機能追加によっては override_method も？) も受け取れるように改修
    let context = PartialReplaceContext::new_root(&fn_info, None, None);
    let stmts_len = f.block.stmts.len();

    let hook_for_tail = return_type_is_result(&f.sig.output);

    f.block
        .stmts
        .iter_mut()
        .enumerate()
        .map(|(i, stmt)| {
            walker::walk_stmt(
                stmt,
                hook_for_tail,
                i == stmts_len - 1,
                &hooq_option,
                &context,
            )
        })
        .collect::<syn::Result<Vec<()>>>()?;

    Ok(quote! {
        #f
    })
}
