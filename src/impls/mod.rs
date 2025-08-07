use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemFn;
use utils::return_type_is_result;

mod inert_attr;
pub mod option;
pub mod utils;
mod walker;

pub use option::HooqOption;

use crate::impls::option::context::{ExtractFunctionInfo, PartialReplaceContext, SkipStatus};
use crate::impls::walker::TailExprTargetKind;

pub fn hooq_impls(mut f: ItemFn) -> syn::Result<TokenStream> {
    let hooq_option = HooqOption::new_from_attrs(&mut f.attrs)?;

    let fn_info = f.extract_function_info()?;
    // TODO: HooqOption の Context への統合
    let skip_status = if hooq_option.is_skiped_all {
        Some(SkipStatus::SkipAll)
    } else {
        None
    };

    let mut context =
        PartialReplaceContext::new_root(&fn_info, skip_status, hooq_option.tag.clone(), None);
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

            walker::walk_stmt(stmt, tail_expr_target_kind, &hooq_option, &context)
        })
        .collect::<syn::Result<Vec<()>>>()?;

    Ok(quote! {
        #f
    })
}
