pub use get_attrs::*;
use syn::spanned::Spanned;
use syn::{ExprClosure, Ident, Path, Signature};

pub mod function_info;
mod get_attrs;

fn path_is_end_of(path: &Path, target: &str) -> bool {
    path.segments
        .iter()
        .next_back()
        .is_some_and(|segment| segment.ident == target)
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

pub fn closure_signature(expr_closure: &ExprClosure) -> Signature {
    let span = expr_closure.span();

    Signature {
        constness: None,
        asyncness: None,
        unsafety: None,
        abi: None,
        fn_token: Default::default(),
        ident: Ident::new("__closure__", span),
        generics: Default::default(),
        paren_token: Default::default(),
        inputs: Default::default(),
        variadic: None,
        output: expr_closure.output.clone(),
    }
}
