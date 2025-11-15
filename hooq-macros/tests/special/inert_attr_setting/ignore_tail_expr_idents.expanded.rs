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
            let path = "<hooq_root>/tests/special/inert_attr_setting/ignore_tail_expr_idents.rs";
            let line = 19usize;
            let col = 17usize;
            let expr = "  19|     enresult(())?\n    |";
            {
                ::std::io::_eprint(
                    format_args!("[{0}:{1}:{2}] {3:?}\n{4}\n", path, line, col, e, expr),
                );
            };
        })?;
    if enresult(false)
        .inspect_err(|e| {
            let path = "<hooq_root>/tests/special/inert_attr_setting/ignore_tail_expr_idents.rs";
            let line = 22usize;
            let col = 23usize;
            let expr = "  22|        enresult(false)?\n    |";
            {
                ::std::io::_eprint(
                    format_args!("[{0}:{1}:{2}] {3:?}\n{4}\n", path, line, col, e, expr),
                );
            };
        })?
    {
        return Right()
            .inspect_err(|e| {
                let path = "<hooq_root>/tests/special/inert_attr_setting/ignore_tail_expr_idents.rs";
                let line = 23usize;
                let col = 9usize;
                let expr = "  23|         return Right()\n    |";
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
                let path = "<hooq_root>/tests/special/inert_attr_setting/ignore_tail_expr_idents.rs";
                let line = 26usize;
                let col = 16usize;
                let expr = "  26|                Right()\n    |";
                {
                    ::std::io::_eprint(
                        format_args!(
                            "[{0}:{1}:{2}] {3:?}\n{4}\n", path, line, col, e, expr
                        ),
                    );
                };
            })
    };
    if enresult(false)
        .inspect_err(|e| {
            let path = "<hooq_root>/tests/special/inert_attr_setting/ignore_tail_expr_idents.rs";
            let line = 29usize;
            let col = 23usize;
            let expr = "  29|        enresult(false)?\n    |";
            {
                ::std::io::_eprint(
                    format_args!("[{0}:{1}:{2}] {3:?}\n{4}\n", path, line, col, e, expr),
                );
            };
        })?
    {
        return Left();
    }
    if enresult(false)
        .inspect_err(|e| {
            let path = "<hooq_root>/tests/special/inert_attr_setting/ignore_tail_expr_idents.rs";
            let line = 34usize;
            let col = 23usize;
            let expr = "  34|        enresult(false)?\n    |";
            {
                ::std::io::_eprint(
                    format_args!("[{0}:{1}:{2}] {3:?}\n{4}\n", path, line, col, e, expr),
                );
            };
        })?
    {
        return Ok(())
            .inspect_err(|e| {
                let path = "<hooq_root>/tests/special/inert_attr_setting/ignore_tail_expr_idents.rs";
                let line = 35usize;
                let col = 9usize;
                let expr = "  35|         return Ok(())\n    |";
                {
                    ::std::io::_eprint(
                        format_args!(
                            "[{0}:{1}:{2}] {3:?}\n{4}\n", path, line, col, e, expr
                        ),
                    );
                };
            });
    }
    if enresult(false)
        .inspect_err(|e| {
            let path = "<hooq_root>/tests/special/inert_attr_setting/ignore_tail_expr_idents.rs";
            let line = 38usize;
            let col = 23usize;
            let expr = "  38|        enresult(false)?\n    |";
            {
                ::std::io::_eprint(
                    format_args!("[{0}:{1}:{2}] {3:?}\n{4}\n", path, line, col, e, expr),
                );
            };
        })?
    {
        return Err(())
            .inspect_err(|e| {
                let path = "<hooq_root>/tests/special/inert_attr_setting/ignore_tail_expr_idents.rs";
                let line = 39usize;
                let col = 9usize;
                let expr = "  39|         return Err(())\n    |";
                {
                    ::std::io::_eprint(
                        format_args!(
                            "[{0}:{1}:{2}] {3:?}\n{4}\n", path, line, col, e, expr
                        ),
                    );
                };
            });
    }
    if enresult(false)
        .inspect_err(|e| {
            let path = "<hooq_root>/tests/special/inert_attr_setting/ignore_tail_expr_idents.rs";
            let line = 42usize;
            let col = 23usize;
            let expr = "  42|        enresult(false)?\n    |";
            {
                ::std::io::_eprint(
                    format_args!("[{0}:{1}:{2}] {3:?}\n{4}\n", path, line, col, e, expr),
                );
            };
        })?
    {
        return enresult(())
            .inspect_err(|e| {
                let path = "<hooq_root>/tests/special/inert_attr_setting/ignore_tail_expr_idents.rs";
                let line = 43usize;
                let col = 9usize;
                let expr = "  43|         return enresult(())\n    |";
                {
                    ::std::io::_eprint(
                        format_args!(
                            "[{0}:{1}:{2}] {3:?}\n{4}\n", path, line, col, e, expr
                        ),
                    );
                };
            });
    }
    Left()
}
fn func2() -> Result<(), ()> {
    #[allow(non_snake_case)]
    let Right = || Result::<(), ()>::Ok(());
    #[allow(non_snake_case)]
    let Left = || Result::<(), ()>::Err(());
    enresult(())
        .inspect_err(|e| {
            let path = "<hooq_root>/tests/special/inert_attr_setting/ignore_tail_expr_idents.rs";
            let line = 60usize;
            let col = 17usize;
            let expr = "  60|     enresult(())?\n    |";
            {
                ::std::io::_eprint(
                    format_args!("[{0}:{1}:{2}] {3:?}\n{4}\n", path, line, col, e, expr),
                );
            };
        })?;
    if enresult(false)
        .inspect_err(|e| {
            let path = "<hooq_root>/tests/special/inert_attr_setting/ignore_tail_expr_idents.rs";
            let line = 62usize;
            let col = 23usize;
            let expr = "  62|        enresult(false)?\n    |";
            {
                ::std::io::_eprint(
                    format_args!("[{0}:{1}:{2}] {3:?}\n{4}\n", path, line, col, e, expr),
                );
            };
        })?
    {
        return Right()
            .inspect_err(|e| {
                let path = "<hooq_root>/tests/special/inert_attr_setting/ignore_tail_expr_idents.rs";
                let line = 63usize;
                let col = 9usize;
                let expr = "  63|         return Right()\n    |";
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
                let path = "<hooq_root>/tests/special/inert_attr_setting/ignore_tail_expr_idents.rs";
                let line = 66usize;
                let col = 16usize;
                let expr = "  66|                Right()\n    |";
                {
                    ::std::io::_eprint(
                        format_args!(
                            "[{0}:{1}:{2}] {3:?}\n{4}\n", path, line, col, e, expr
                        ),
                    );
                };
            })
    };
    if enresult(false)
        .inspect_err(|e| {
            let path = "<hooq_root>/tests/special/inert_attr_setting/ignore_tail_expr_idents.rs";
            let line = 68usize;
            let col = 23usize;
            let expr = "  68|        enresult(false)?\n    |";
            {
                ::std::io::_eprint(
                    format_args!("[{0}:{1}:{2}] {3:?}\n{4}\n", path, line, col, e, expr),
                );
            };
        })?
    {
        return Left();
    }
    if enresult(false)
        .inspect_err(|e| {
            let path = "<hooq_root>/tests/special/inert_attr_setting/ignore_tail_expr_idents.rs";
            let line = 72usize;
            let col = 23usize;
            let expr = "  72|        enresult(false)?\n    |";
            {
                ::std::io::_eprint(
                    format_args!("[{0}:{1}:{2}] {3:?}\n{4}\n", path, line, col, e, expr),
                );
            };
        })?
    {
        return Ok(())
            .inspect_err(|e| {
                let path = "<hooq_root>/tests/special/inert_attr_setting/ignore_tail_expr_idents.rs";
                let line = 73usize;
                let col = 9usize;
                let expr = "  73|         return Ok(())\n    |";
                {
                    ::std::io::_eprint(
                        format_args!(
                            "[{0}:{1}:{2}] {3:?}\n{4}\n", path, line, col, e, expr
                        ),
                    );
                };
            });
    }
    if enresult(false)
        .inspect_err(|e| {
            let path = "<hooq_root>/tests/special/inert_attr_setting/ignore_tail_expr_idents.rs";
            let line = 76usize;
            let col = 23usize;
            let expr = "  76|        enresult(false)?\n    |";
            {
                ::std::io::_eprint(
                    format_args!("[{0}:{1}:{2}] {3:?}\n{4}\n", path, line, col, e, expr),
                );
            };
        })?
    {
        return Err(())
            .inspect_err(|e| {
                let path = "<hooq_root>/tests/special/inert_attr_setting/ignore_tail_expr_idents.rs";
                let line = 77usize;
                let col = 9usize;
                let expr = "  77|         return Err(())\n    |";
                {
                    ::std::io::_eprint(
                        format_args!(
                            "[{0}:{1}:{2}] {3:?}\n{4}\n", path, line, col, e, expr
                        ),
                    );
                };
            });
    }
    if enresult(false)
        .inspect_err(|e| {
            let path = "<hooq_root>/tests/special/inert_attr_setting/ignore_tail_expr_idents.rs";
            let line = 80usize;
            let col = 23usize;
            let expr = "  80|        enresult(false)?\n    |";
            {
                ::std::io::_eprint(
                    format_args!("[{0}:{1}:{2}] {3:?}\n{4}\n", path, line, col, e, expr),
                );
            };
        })?
    {
        return enresult(())
            .inspect_err(|e| {
                let path = "<hooq_root>/tests/special/inert_attr_setting/ignore_tail_expr_idents.rs";
                let line = 81usize;
                let col = 9usize;
                let expr = "  81|         return enresult(())\n    |";
                {
                    ::std::io::_eprint(
                        format_args!(
                            "[{0}:{1}:{2}] {3:?}\n{4}\n", path, line, col, e, expr
                        ),
                    );
                };
            });
    }
    Left()
}
