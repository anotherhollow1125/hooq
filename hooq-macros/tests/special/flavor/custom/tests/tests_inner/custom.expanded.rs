use std::fmt::Debug;
use hooq::{hooq, toml_load};
#[allow(unused)]
fn enresult<T: Debug>(val: T) -> Result<T, ()> {
    Ok(val)
        .inspect(|v| {
            ::std::io::_print(
                format_args!("Ok Value with: {1:?} & with tag: {0}\n", "[default]", v),
            );
        })
        .inspect_err(|e| {
            ::std::io::_eprint(
                format_args!("Err Value with: {1:?} & with tag: {0}\n", "[default]", e),
            );
        })
}
#[allow(unused)]
fn func(flag: bool) -> Result<(), ()> {
    enresult(())
        .inspect(|v| {
            ::std::io::_print(
                format_args!("Ok Value with: {1:?} & with tag: {0}\n", "[sub]", v),
            );
        })
        .inspect_err(|e| {
            ::std::io::_eprint(
                format_args!("Err Value with: {1:?} & with tag: {0}\n", "[sub]", e),
            );
        })?;
    if flag {
        return Ok(())
            .inspect(|v| {
                ::std::io::_print(
                    format_args!("Ok Value with: {1:?} & with tag: {0}\n", "[sub]", v),
                );
            })
            .inspect_err(|e| {
                ::std::io::_eprint(
                    format_args!("Err Value with: {1:?} & with tag: {0}\n", "[sub]", e),
                );
            });
    }
    let _ = { Result::<(), ()>::Ok(()) };
    Err(())
        .inspect(|v| {
            ::std::io::_print(
                format_args!("Ok Value with: {1:?} & with tag: {0}\n", "[sub]", v),
            );
        })
        .inspect_err(|e| {
            ::std::io::_eprint(
                format_args!("Err Value with: {1:?} & with tag: {0}\n", "[sub]", e),
            );
        })
}
#[allow(unused)]
fn func2(flag: bool) -> Result<(), ()> {
    enresult(())
        .inspect(|v| {
            ::std::io::_print(
                format_args!("Ok Value with: {1:?} & with tag: {0}\n", "[sub.sub]", v),
            );
        })
        .inspect_err(|e| {
            ::std::io::_eprint(
                format_args!("Err Value with: {1:?} & with tag: {0}\n", "[sub.sub]", e),
            );
        })?;
    if flag {
        return Ok(())
            .inspect(|v| {
                ::std::io::_print(
                    format_args!(
                        "Ok Value with: {1:?} & with tag: {0}\n", "[sub.sub]", v
                    ),
                );
            })
            .inspect_err(|e| {
                ::std::io::_eprint(
                    format_args!(
                        "Err Value with: {1:?} & with tag: {0}\n", "[sub.sub]", e
                    ),
                );
            });
    }
    let res = {
        Ok(())
            .inspect(|v| {
                ::std::io::_print(
                    format_args!(
                        "Ok Value with: {1:?} & with tag: {0}\n", "[sub.sub]", v
                    ),
                );
            })
            .inspect_err(|e| {
                ::std::io::_eprint(
                    format_args!(
                        "Err Value with: {1:?} & with tag: {0}\n", "[sub.sub]", e
                    ),
                );
            })
    };
    {
        ::std::io::_print(format_args!("res: {0:?}\n", res));
    };
    res.inspect(|v| {
            ::std::io::_print(
                format_args!("Ok Value with: {1:?} & with tag: {0}\n", "[sub.sub]", v),
            );
        })
        .inspect_err(|e| {
            ::std::io::_eprint(
                format_args!("Err Value with: {1:?} & with tag: {0}\n", "[sub.sub]", e),
            );
        })
}
#[allow(unused)]
fn func3(flag: bool) -> Result<(), ()> {
    enresult(())
        .inspect(|v| {
            ::std::io::_print(
                format_args!(
                    "Ok Value with: {1:?} & with tag: {0}\n",
                    "[not_tail_expr_idents_test_1]", v
                ),
            );
        })
        .inspect_err(|e| {
            ::std::io::_eprint(
                format_args!(
                    "Err Value with: {1:?} & with tag: {0}\n",
                    "[not_tail_expr_idents_test_1]", e
                ),
            );
        })?;
    if flag {
        return Ok(());
    }
    let res = { Ok(()) };
    {
        ::std::io::_print(format_args!("res: {0:?}\n", res));
    };
    res.inspect(|v| {
            ::std::io::_print(
                format_args!(
                    "Ok Value with: {1:?} & with tag: {0}\n",
                    "[not_tail_expr_idents_test_1]", v
                ),
            );
        })
        .inspect_err(|e| {
            ::std::io::_eprint(
                format_args!(
                    "Err Value with: {1:?} & with tag: {0}\n",
                    "[not_tail_expr_idents_test_1]", e
                ),
            );
        })
}
#[allow(unused)]
fn func4(flag: bool) -> Result<(), ()> {
    enresult(())
        .inspect(|v| {
            ::std::io::_print(
                format_args!(
                    "Ok Value with: {1:?} & with tag: {0}\n",
                    "[not_tail_expr_idents_test_2]", v
                ),
            );
        })
        .inspect_err(|e| {
            ::std::io::_eprint(
                format_args!(
                    "Err Value with: {1:?} & with tag: {0}\n",
                    "[not_tail_expr_idents_test_2]", e
                ),
            );
        })?;
    if flag {
        return Ok(());
    }
    let res = { Ok(()) };
    {
        ::std::io::_print(format_args!("res: {0:?}\n", res));
    };
    res.inspect(|v| {
            ::std::io::_print(
                format_args!(
                    "Ok Value with: {1:?} & with tag: {0}\n",
                    "[not_tail_expr_idents_test_2]", v
                ),
            );
        })
        .inspect_err(|e| {
            ::std::io::_eprint(
                format_args!(
                    "Err Value with: {1:?} & with tag: {0}\n",
                    "[not_tail_expr_idents_test_2]", e
                ),
            );
        })
}
