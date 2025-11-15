use hooq_macros::hooq;
fn enresult<T>(val: T) -> Result<T, ()> {
    Ok(val)
}
fn func() -> Result<(), ()> {
    #[allow(non_snake_case)]
    let Right = || Result::<(), ()>::Ok(());
    #[allow(non_snake_case)]
    let Left = || Result::<(), ()>::Err(());
    enresult(())
        .inspect_err(|e| {
            let path = "<hooq_root>/tests/special/inert_attr_setting/tail_expr_idents.rs";
            let line = 24usize;
            let col = 17usize;
            let expr = "  24>    enresult(())?\n    |";
            {
                ::std::io::_eprint(
                    format_args!("[{0}:{1}:{2}] {3:?}\n{4}\n", path, line, col, e, expr),
                );
            };
        })?;
    if enresult(false)
        .inspect_err(|e| {
            let path = "<hooq_root>/tests/special/inert_attr_setting/tail_expr_idents.rs";
            let line = 27usize;
            let col = 23usize;
            let expr = "  27>    enresult(false)?\n    |";
            {
                ::std::io::_eprint(
                    format_args!("[{0}:{1}:{2}] {3:?}\n{4}\n", path, line, col, e, expr),
                );
            };
        })?
    {
        return Ok(());
    }
    if enresult(false)
        .inspect_err(|e| {
            let path = "<hooq_root>/tests/special/inert_attr_setting/tail_expr_idents.rs";
            let line = 31usize;
            let col = 23usize;
            let expr = "  31>    enresult(false)?\n    |";
            {
                ::std::io::_eprint(
                    format_args!("[{0}:{1}:{2}] {3:?}\n{4}\n", path, line, col, e, expr),
                );
            };
        })?
    {
        return Err(());
    }
    if enresult(false)
        .inspect_err(|e| {
            let path = "<hooq_root>/tests/special/inert_attr_setting/tail_expr_idents.rs";
            let line = 36usize;
            let col = 23usize;
            let expr = "  36>    enresult(false)?\n    |";
            {
                ::std::io::_eprint(
                    format_args!("[{0}:{1}:{2}] {3:?}\n{4}\n", path, line, col, e, expr),
                );
            };
        })?
    {
        return Left()
            .inspect_err(|e| {
                let path = "<hooq_root>/tests/special/inert_attr_setting/tail_expr_idents.rs";
                let line = 37usize;
                let col = 9usize;
                let expr = "  37>    return Left()\n    |";
                {
                    ::std::io::_eprint(
                        format_args!(
                            "[{0}:{1}:{2}] {3:?}\n{4}\n", path, line, col, e, expr
                        ),
                    );
                };
            });
    }
    let _ = || {
        Right()
            .inspect_err(|e| {
                let path = "<hooq_root>/tests/special/inert_attr_setting/tail_expr_idents.rs";
                let line = 40usize;
                let col = 16usize;
                let expr = "  40>    Right()\n    |";
                {
                    ::std::io::_eprint(
                        format_args!(
                            "[{0}:{1}:{2}] {3:?}\n{4}\n", path, line, col, e, expr
                        ),
                    );
                };
            })
    };
    Right()
        .inspect_err(|e| {
            let path = "<hooq_root>/tests/special/inert_attr_setting/tail_expr_idents.rs";
            let line = 42usize;
            let col = 5usize;
            let expr = "  42>    Right()\n    |";
            {
                ::std::io::_eprint(
                    format_args!("[{0}:{1}:{2}] {3:?}\n{4}\n", path, line, col, e, expr),
                );
            };
        })
}
