use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemFn;
use utils::return_type_is_result;

pub mod attr;
pub mod utils;
mod walker;

use crate::impls::attr::context::{ExtractFunctionInfo, HookContext};
use crate::impls::attr::inert_attr::extract_hooq_info_from_attrs;
use crate::impls::walker::TailExprTargetKind;

pub fn hooq_impls(mut f: ItemFn) -> syn::Result<TokenStream> {
    let fn_info = f.extract_function_info()?;

    let inert_attr_option = extract_hooq_info_from_attrs(&mut f.attrs)?;
    let mut context = HookContext::init(&fn_info, inert_attr_option);
    let stmts_len = f.block.stmts.len();

    context.update_return_type_is_result(return_type_is_result(&f.sig.output));

    f.block
        .stmts
        .iter_mut()
        .enumerate()
        .map(|(i, stmt)| {
            let tail_expr_target_kind = if i == stmts_len - 1 {
                TailExprTargetKind::FnBlockTailExpr
            } else {
                TailExprTargetKind::NotTarget
            };

            walker::walk_stmt(stmt, tail_expr_target_kind, &context)
        })
        .collect::<syn::Result<Vec<()>>>()?;

    Ok(quote! {
        #f
    })
}
