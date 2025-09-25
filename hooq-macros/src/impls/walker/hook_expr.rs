use proc_macro2::Span;
use syn::{Expr, parse_quote};

use crate::impls::inert_attr::context::{HookContext, HookTargetKind};

pub(super) fn hook_expr(
    apply: bool,
    expr_field: &mut Expr,
    expr_field_for_display: &str,
    kind: HookTargetKind,
    q_span: Span,
    context: &HookContext,
) -> syn::Result<()> {
    if !apply {
        return Ok(());
    }

    context.counter.borrow_mut().count_up(kind);
    let context = context.as_hook_info(expr_field_for_display, kind);

    if !context.is_hook_target() {
        return Ok(());
    }

    let method = context.generate_method(q_span)?;
    let original_expr = expr_field.clone();

    *expr_field = parse_quote! {
        #original_expr #method
    };

    Ok(())
}
