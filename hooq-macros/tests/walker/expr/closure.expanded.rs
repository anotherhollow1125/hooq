use hooq_macros::hooq;
fn minus(x: u32, y: u32) -> Result<u32, ()> {
    if y > x {
        return Err(())
            .inspect_err(|e| {
                let path = "<hooq_root>/tests/walker/expr/closure.rs";
                let line = 6usize;
                {
                    ::std::io::_eprint(
                        format_args!("[{0}:L{1}] {2:?}\n", path, line, e),
                    );
                };
            });
    }
    Ok(x - y)
        .inspect_err(|e| {
            let path = "<hooq_root>/tests/walker/expr/closure.rs";
            let line = 8usize;
            {
                ::std::io::_eprint(format_args!("[{0}:L{1}] {2:?}\n", path, line, e));
            };
        })
}
fn two() -> Result<u32, ()> {
    Ok(2)
        .inspect_err(|e| {
            let path = "<hooq_root>/tests/walker/expr/closure.rs";
            let line = 13usize;
            {
                ::std::io::_eprint(format_args!("[{0}:L{1}] {2:?}\n", path, line, e));
            };
        })
}
fn func() -> Result<(), ()> {
    let _ = |v| {
        minus(v, 1)
            .inspect(|_| {
                {
                    ::std::io::_print(
                        format_args!("tag: {0}\n", "No Result Type (Block)"),
                    );
                };
            })?;
        minus(
            v,
            two()
                .inspect(|_| {
                    {
                        ::std::io::_print(
                            format_args!("tag: {0}\n", "No Result Type (Block)"),
                        );
                    };
                })?,
        )
    };
    let _ = |v: u32| -> Result<u32, ()> {
        minus(v, 1)
            .inspect(|_| {
                {
                    ::std::io::_print(
                        format_args!("tag: {0}\n", "Result Type Annotated (Block)"),
                    );
                };
            })?;
        minus(
                v,
                two()
                    .inspect(|_| {
                        {
                            ::std::io::_print(
                                format_args!("tag: {0}\n", "Result Type Annotated (Block)"),
                            );
                        };
                    })?,
            )
            .inspect(|_| {
                {
                    ::std::io::_print(
                        format_args!("tag: {0}\n", "Result Type Annotated (Block)"),
                    );
                };
            })
    };
    let _ = |v| {
        minus(v, 1)
            .inspect(|_| {
                {
                    ::std::io::_print(
                        format_args!("tag: {0}\n", "Expr is Result Type (Block)"),
                    );
                };
            })?;
        minus(
                v,
                two()
                    .inspect(|_| {
                        {
                            ::std::io::_print(
                                format_args!("tag: {0}\n", "Expr is Result Type (Block)"),
                            );
                        };
                    })?,
            )
            .inspect(|_| {
                {
                    ::std::io::_print(
                        format_args!("tag: {0}\n", "Expr is Result Type (Block)"),
                    );
                };
            })?;
        if v > 100 {
            return Err(())
                .inspect(|_| {
                    {
                        ::std::io::_print(
                            format_args!("tag: {0}\n", "Expr is Result Type (Block)"),
                        );
                    };
                });
        }
        Ok(
                minus(v, 3)
                    .inspect(|_| {
                        {
                            ::std::io::_print(
                                format_args!("tag: {0}\n", "Expr is Result Type (Block)"),
                            );
                        };
                    })? + 1,
            )
            .inspect(|_| {
                {
                    ::std::io::_print(
                        format_args!("tag: {0}\n", "Expr is Result Type (Block)"),
                    );
                };
            })
    };
    let _ = |v| minus(
        v,
        two()
            .inspect(|_| {
                {
                    ::std::io::_print(
                        format_args!("tag: {0}\n", "No Result Type (Single Expr)"),
                    );
                };
            })?,
    );
    let _ = |v| {
        Ok({
                if v > 100 {
                    return Err(())
                        .inspect(|_| {
                            {
                                ::std::io::_print(
                                    format_args!(
                                        "tag: {0}\n", "Expr is Result Type (Single Expr)"
                                    ),
                                );
                            };
                        });
                }
                minus(v, 1)
                    .inspect(|_| {
                        {
                            ::std::io::_print(
                                format_args!(
                                    "tag: {0}\n", "Expr is Result Type (Single Expr)"
                                ),
                            );
                        };
                    })?
                    + minus(
                            v,
                            two()
                                .inspect(|_| {
                                    {
                                        ::std::io::_print(
                                            format_args!(
                                                "tag: {0}\n", "Expr is Result Type (Single Expr)"
                                            ),
                                        );
                                    };
                                })?,
                        )
                        .inspect(|_| {
                            {
                                ::std::io::_print(
                                    format_args!(
                                        "tag: {0}\n", "Expr is Result Type (Single Expr)"
                                    ),
                                );
                            };
                        })? + 1
            })
            .inspect(|_| {
                {
                    ::std::io::_print(
                        format_args!("tag: {0}\n", "Expr is Result Type (Single Expr)"),
                    );
                };
            })
    };
    Ok(())
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
            };
        })
}
