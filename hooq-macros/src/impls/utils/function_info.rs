use std::ops::Deref;

use quote::ToTokens;
use syn::{ReturnType, Signature, Type, TypePath};

use crate::impls::utils::path_is_end_of;

#[derive(Clone, Debug)]
pub struct FunctionInfo(Signature);

impl FunctionInfo {
    pub fn new(sig: Signature) -> Self {
        Self(sig)
    }

    pub fn name(&self) -> String {
        self.0.ident.to_string()
    }

    pub fn sig(&self) -> String {
        self.0.to_token_stream().to_string()
    }

    pub fn return_type_is_result(&self) -> bool {
        return_type_is_result_inner(&self.0.output)
    }
}

fn return_type_is_result_inner(rt: &ReturnType) -> bool {
    if let ReturnType::Type(_, t) = rt
        && let Type::Path(TypePath { path, .. }) = t.deref()
    {
        path_is_end_of(path, "Result")
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
        assert!(return_type_is_result_inner(&item_fn.sig.output));

        let item_fn: ItemFn = parse_quote! {
            fn bar() -> ::std::result::Result<(), ()> {
                Ok(())
            }
        };
        assert!(return_type_is_result_inner(&item_fn.sig.output));

        let item_fn: ItemFn = parse_quote! {
            fn baz() -> i32 {
                42
            }
        };
        assert!(!return_type_is_result_inner(&item_fn.sig.output));
    }
}
