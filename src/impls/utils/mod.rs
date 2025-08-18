use std::ops::Deref;

pub use get_attrs::*;
use syn::{Path, ReturnType, Type, TypePath};

mod get_attrs;

fn path_is_end_of(path: &Path, target: &str) -> bool {
    path.segments
        .iter()
        .next_back()
        .is_some_and(|segment| segment.ident == target)
}

pub fn return_type_is_result(rt: &ReturnType) -> bool {
    if let ReturnType::Type(_, t) = rt
        && let Type::Path(TypePath { path, .. }) = t.deref()
    {
        path_is_end_of(path, "Result")
    } else {
        false
    }
}

pub fn path_is_special_call_like_err(path: &Path) -> bool {
    #[cfg(not(feature = "err-only"))]
    {
        path_is_end_of(path, "Err") || path_is_end_of(path, "Ok")
    }
    #[cfg(feature = "err-only")]
    {
        path_is_end_of(path, "Err")
    }
}

#[cfg(test)]
mod tests {
    use syn::{ItemFn, parse_quote};

    use super::return_type_is_result;

    #[test]
    fn test_return_type_is_result() {
        let item_fn: ItemFn = parse_quote! {
            fn foo() -> Result<(), ()> {
                Ok(())
            }
        };
        assert!(return_type_is_result(&item_fn.sig.output));

        let item_fn: ItemFn = parse_quote! {
            fn bar() -> ::std::result::Result<(), ()> {
                Ok(())
            }
        };
        assert!(return_type_is_result(&item_fn.sig.output));

        let item_fn: ItemFn = parse_quote! {
            fn baz() -> i32 {
                42
            }
        };
        assert!(!return_type_is_result(&item_fn.sig.output));
    }
}
