use proc_macro2::{Span, TokenStream};
use syn::{Expr, Token};

use crate::impls::inert_attr::context::{HookContext, HookTargetKind};

pub(super) fn hook_expr(
    apply: bool,
    expr_field: &mut Expr,
    source_tokenstream: TokenStream,
    kind: HookTargetKind,
    q_span: Span,
    context: &HookContext,
) -> syn::Result<Option<Token![!]>> {
    if !apply {
        return Ok(None);
    }

    context.counter.borrow_mut().count_up(kind);
    let context = context.as_hook_info(source_tokenstream, kind);

    if !context.is_hook_target() {
        return Ok(None);
    }

    let exc = context.render_expr_with_method(expr_field, q_span)?;

    Ok(exc)
}
