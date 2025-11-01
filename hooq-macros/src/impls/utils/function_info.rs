use std::ops::Deref;
use std::rc::Rc;

use quote::ToTokens;
use syn::{ExprClosure, ReturnType, Signature, Type, TypePath, parse_quote};

use crate::impls::inert_attr::context::LocalContextField;
use crate::impls::utils::path_is_end_of;

#[derive(Clone, Debug)]
pub struct ClosureInfo {
    expr: ExprClosure,
    name: String,
}

impl ClosureInfo {
    pub fn new(
        mut expr: ExprClosure,
        current: Option<Rc<LocalContextField<Option<FunctionInfo>>>>,
    ) -> Self {
        expr.body = Box::new(parse_quote! { {} });

        let Some(current) = current else {
            return ClosureInfo {
                expr,
                name: "__closure_in_<unknown>__".to_string(),
            };
        };

        let mut ptr = current.clone();
        let mut ancestor_function_name = match &**ptr {
            Some(FunctionInfo::Function(sig)) => Some(sig.ident.to_string()),
            _ => None,
        };
        while ancestor_function_name.is_none()
            && let Some(parent) = ptr.get_overridden_ancestor()
        {
            ancestor_function_name = match &**parent {
                Some(FunctionInfo::Function(sig)) => Some(sig.ident.to_string()),
                _ => None,
            };
            ptr = parent;
        }

        let anc_name = ancestor_function_name.unwrap_or_else(|| "<unknown>".to_string());
        let name = format!("__closure_in_{}__", anc_name);

        ClosureInfo { expr, name }
    }
}

#[derive(Clone, Debug)]
pub enum FunctionInfo {
    Function(Signature),
    Closure(ClosureInfo),
}

impl From<ClosureInfo> for FunctionInfo {
    fn from(value: ClosureInfo) -> Self {
        FunctionInfo::Closure(value)
    }
}

impl FunctionInfo {
    pub fn name(&self) -> String {
        match self {
            FunctionInfo::Function(signature) => signature.ident.to_string(),
            FunctionInfo::Closure(closure_info) => closure_info.name.clone(),
        }
    }

    pub fn sig(&self) -> String {
        match self {
            FunctionInfo::Function(signature) => signature.to_token_stream().to_string(),
            FunctionInfo::Closure(closure_info) => closure_info.expr.to_token_stream().to_string(),
        }
    }

    pub fn return_type_is_result(&self, result_types: &[String]) -> bool {
        let output = match self {
            FunctionInfo::Function(signature) => &signature.output,
            FunctionInfo::Closure(closure_info) => &closure_info.expr.output,
        };

        return_type_is_result_inner(output, result_types)
    }
}

fn return_type_is_result_inner(rt: &ReturnType, result_types: &[String]) -> bool {
    if let ReturnType::Type(_, t) = rt
        && let Type::Path(TypePath { path, .. }) = t.deref()
    {
        result_types.iter().any(|s| path_is_end_of(path, s))
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use syn::{ItemFn, parse_quote};

    use super::return_type_is_result_inner;

    #[test]
    fn test_return_type_is_result() {
        let item_fn: ItemFn = parse_quote! {
            fn foo() -> Result<(), ()> {
                Ok(())
            }
        };
        assert!(return_type_is_result_inner(
            &item_fn.sig.output,
            &["Result".to_string()]
        ));

        let item_fn: ItemFn = parse_quote! {
            fn bar() -> ::std::result::Result<(), ()> {
                Ok(())
            }
        };
        assert!(return_type_is_result_inner(
            &item_fn.sig.output,
            &["Result".to_string()]
        ));

        let item_fn: ItemFn = parse_quote! {
            fn baz() -> i32 {
                42
            }
        };
        assert!(!return_type_is_result_inner(
            &item_fn.sig.output,
            &["Result".to_string()]
        ));
    }
}
