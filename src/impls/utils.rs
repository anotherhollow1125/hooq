use std::ops::Deref;

use syn::{
    Attribute, Expr, ExprArray, ExprAssign, ExprAsync, ExprAwait, ExprBinary, ExprBlock, ExprBreak,
    ExprCall, ExprCast, ExprClosure, ExprConst, ExprContinue, ExprField, ExprForLoop, ExprGroup,
    ExprIf, ExprIndex, ExprInfer, ExprLet, ExprLit, ExprLoop, ExprMacro, ExprMatch, ExprMethodCall,
    ExprParen, ExprPath, ExprRange, ExprRawAddr, ExprReference, ExprRepeat, ExprReturn, ExprStruct,
    ExprTry, ExprTryBlock, ExprTuple, ExprUnary, ExprUnsafe, ExprWhile, ExprYield, Path,
    ReturnType, Type, TypePath,
};

pub fn get_attrs_from_expr(expr: &mut Expr) -> Option<&mut Vec<Attribute>> {
    match expr {
        Expr::Array(ExprArray { attrs, .. })
        | Expr::Assign(ExprAssign { attrs, .. })
        | Expr::Async(ExprAsync { attrs, .. })
        | Expr::Await(ExprAwait { attrs, .. })
        | Expr::Binary(ExprBinary { attrs, .. })
        | Expr::Block(ExprBlock { attrs, .. })
        | Expr::Break(ExprBreak { attrs, .. })
        | Expr::Call(ExprCall { attrs, .. })
        | Expr::Cast(ExprCast { attrs, .. })
        | Expr::Closure(ExprClosure { attrs, .. })
        | Expr::Const(ExprConst { attrs, .. })
        | Expr::Continue(ExprContinue { attrs, .. })
        | Expr::Field(ExprField { attrs, .. })
        | Expr::ForLoop(ExprForLoop { attrs, .. })
        | Expr::Group(ExprGroup { attrs, .. })
        | Expr::If(ExprIf { attrs, .. })
        | Expr::Index(ExprIndex { attrs, .. })
        | Expr::Infer(ExprInfer { attrs, .. })
        | Expr::Let(ExprLet { attrs, .. })
        | Expr::Lit(ExprLit { attrs, .. })
        | Expr::Loop(ExprLoop { attrs, .. })
        | Expr::Macro(ExprMacro { attrs, .. })
        | Expr::Match(ExprMatch { attrs, .. })
        | Expr::MethodCall(ExprMethodCall { attrs, .. })
        | Expr::Paren(ExprParen { attrs, .. })
        | Expr::Path(ExprPath { attrs, .. })
        | Expr::Range(ExprRange { attrs, .. })
        | Expr::RawAddr(ExprRawAddr { attrs, .. })
        | Expr::Reference(ExprReference { attrs, .. })
        | Expr::Repeat(ExprRepeat { attrs, .. })
        | Expr::Return(ExprReturn { attrs, .. })
        | Expr::Struct(ExprStruct { attrs, .. })
        | Expr::Try(ExprTry { attrs, .. })
        | Expr::TryBlock(ExprTryBlock { attrs, .. })
        | Expr::Tuple(ExprTuple { attrs, .. })
        | Expr::Unary(ExprUnary { attrs, .. })
        | Expr::Unsafe(ExprUnsafe { attrs, .. })
        | Expr::While(ExprWhile { attrs, .. })
        | Expr::Yield(ExprYield { attrs, .. }) => Some(attrs),
        Expr::Verbatim(_) | _ => None,
    }
}

fn path_is_end_of(path: &Path, target: &str) -> bool {
    path.segments
        .iter()
        .next_back()
        .is_some_and(|segment| segment.ident == target)
}

pub fn return_type_is_result(rt: &ReturnType) -> bool {
    if let ReturnType::Type(_, t) = rt {
        if let Type::Path(TypePath { path, .. }) = t.deref() {
            path_is_end_of(path, "Result")
        } else {
            false
        }
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
