use hooq_macros::hooq;
mod funcs {
    type Either = Result<(), ()>;
    type NotTarget = Result<(), ()>;
    fn enresult<T>(val: T) -> Result<T, ()> {
        Ok(val)
            .inspect_err(|e| {
                let path = "<hooq_root>/tests/special/inert_attr_setting/result_types.rs";
                let line = 12usize;
                let col = 9usize;
                let expr = "Ok(val)";
                {
                    ::std::io::_eprint(
                        format_args!(
                            "[{0}:{1}:{2}] {3:?}\n    {4}\n", path, line, col, e, expr
                        ),
                    );
                };
            })
    }
    pub fn result_fn() -> Result<(), ()> {
        if enresult(false)
            .inspect_err(|e| {
                let path = "<hooq_root>/tests/special/inert_attr_setting/result_types.rs";
                let line = 16usize;
                let col = 27usize;
                let expr = "enresult(false)?";
                {
                    ::std::io::_eprint(
                        format_args!(
                            "[{0}:{1}:{2}] {3:?}\n    {4}\n", path, line, col, e, expr
                        ),
                    );
                };
            })?
        {
            return enresult(())
                .inspect_err(|e| {
                    let path = "<hooq_root>/tests/special/inert_attr_setting/result_types.rs";
                    let line = 17usize;
                    let col = 13usize;
                    let expr = "return enresult(());";
                    {
                        ::std::io::_eprint(
                            format_args!(
                                "[{0}:{1}:{2}] {3:?}\n    {4}\n", path, line, col, e, expr
                            ),
                        );
                    };
                });
        }
        enresult(())
            .inspect_err(|e| {
                let path = "<hooq_root>/tests/special/inert_attr_setting/result_types.rs";
                let line = 20usize;
                let col = 9usize;
                let expr = "enresult(())";
                {
                    ::std::io::_eprint(
                        format_args!(
                            "[{0}:{1}:{2}] {3:?}\n    {4}\n", path, line, col, e, expr
                        ),
                    );
                };
            })
    }
    pub fn either_fn() -> Either {
        if enresult(false)
            .inspect_err(|e| {
                let path = "<hooq_root>/tests/special/inert_attr_setting/result_types.rs";
                let line = 24usize;
                let col = 27usize;
                let expr = "enresult(false)?";
                {
                    ::std::io::_eprint(
                        format_args!(
                            "[{0}:{1}:{2}] {3:?}\n    {4}\n", path, line, col, e, expr
                        ),
                    );
                };
            })?
        {
            return enresult(())
                .inspect_err(|e| {
                    let path = "<hooq_root>/tests/special/inert_attr_setting/result_types.rs";
                    let line = 25usize;
                    let col = 13usize;
                    let expr = "return enresult(());";
                    {
                        ::std::io::_eprint(
                            format_args!(
                                "[{0}:{1}:{2}] {3:?}\n    {4}\n", path, line, col, e, expr
                            ),
                        );
                    };
                });
        }
        enresult(())
            .inspect_err(|e| {
                let path = "<hooq_root>/tests/special/inert_attr_setting/result_types.rs";
                let line = 28usize;
                let col = 9usize;
                let expr = "enresult(())";
                {
                    ::std::io::_eprint(
                        format_args!(
                            "[{0}:{1}:{2}] {3:?}\n    {4}\n", path, line, col, e, expr
                        ),
                    );
                };
            })
    }
    pub fn other_fn_1() -> NotTarget {
        if enresult(false)
            .inspect_err(|e| {
                let path = "<hooq_root>/tests/special/inert_attr_setting/result_types.rs";
                let line = 32usize;
                let col = 27usize;
                let expr = "enresult(false)?";
                {
                    ::std::io::_eprint(
                        format_args!(
                            "[{0}:{1}:{2}] {3:?}\n    {4}\n", path, line, col, e, expr
                        ),
                    );
                };
            })?
        {
            return enresult(());
        }
        Err(())
    }
    pub fn other_fn_2() -> NotTarget {
        if enresult(false)
            .inspect_err(|e| {
                let path = "<hooq_root>/tests/special/inert_attr_setting/result_types.rs";
                let line = 45usize;
                let col = 27usize;
                let expr = "enresult(false)?";
                {
                    ::std::io::_eprint(
                        format_args!(
                            "[{0}:{1}:{2}] {3:?}\n    {4}\n", path, line, col, e, expr
                        ),
                    );
                };
            })?
        {
            return enresult(());
        }
        if enresult(false)
            .inspect_err(|e| {
                let path = "<hooq_root>/tests/special/inert_attr_setting/result_types.rs";
                let line = 49usize;
                let col = 27usize;
                let expr = "enresult(false)?";
                {
                    ::std::io::_eprint(
                        format_args!(
                            "[{0}:{1}:{2}] {3:?}\n    {4}\n", path, line, col, e, expr
                        ),
                    );
                };
            })?
        {
            return Ok(())
                .inspect_err(|e| {
                    let path = "<hooq_root>/tests/special/inert_attr_setting/result_types.rs";
                    let line = 50usize;
                    let col = 13usize;
                    let expr = "return Ok(());";
                    {
                        ::std::io::_eprint(
                            format_args!(
                                "[{0}:{1}:{2}] {3:?}\n    {4}\n", path, line, col, e, expr
                            ),
                        );
                    };
                });
        }
        Ok(())
            .inspect_err(|e| {
                let path = "<hooq_root>/tests/special/inert_attr_setting/result_types.rs";
                let line = 53usize;
                let col = 9usize;
                let expr = "Ok(())";
                {
                    ::std::io::_eprint(
                        format_args!(
                            "[{0}:{1}:{2}] {3:?}\n    {4}\n", path, line, col, e, expr
                        ),
                    );
                };
            })
    }
    pub fn other_fn_3() -> NotTarget {
        if enresult(false)
            .inspect_err(|e| {
                let path = "<hooq_root>/tests/special/inert_attr_setting/result_types.rs";
                let line = 57usize;
                let col = 27usize;
                let expr = "enresult(false)?";
                {
                    ::std::io::_eprint(
                        format_args!(
                            "[{0}:{1}:{2}] {3:?}\n    {4}\n", path, line, col, e, expr
                        ),
                    );
                };
            })?
        {
            return enresult(());
        }
        if enresult(false)
            .inspect_err(|e| {
                let path = "<hooq_root>/tests/special/inert_attr_setting/result_types.rs";
                let line = 61usize;
                let col = 27usize;
                let expr = "enresult(false)?";
                {
                    ::std::io::_eprint(
                        format_args!(
                            "[{0}:{1}:{2}] {3:?}\n    {4}\n", path, line, col, e, expr
                        ),
                    );
                };
            })?
        {
            return Err(());
        }
        Ok(())
            .inspect_err(|e| {
                let path = "<hooq_root>/tests/special/inert_attr_setting/result_types.rs";
                let line = 67usize;
                let col = 9usize;
                let expr = "Ok(())";
                {
                    ::std::io::_eprint(
                        format_args!(
                            "[{0}:{1}:{2}] {3:?}\n    {4}\n", path, line, col, e, expr
                        ),
                    );
                };
            })
    }
    pub fn other_fn_4_1() -> Either {
        if enresult(false)
            .inspect_err(|e| {
                let path = "<hooq_root>/tests/special/inert_attr_setting/result_types.rs";
                let line = 75usize;
                let col = 27usize;
                let expr = "enresult(false)?";
                {
                    ::std::io::_eprint(
                        format_args!(
                            "[{0}:{1}:{2}] {3:?}\n    {4}\n", path, line, col, e, expr
                        ),
                    );
                };
            })?
        {
            return enresult(())
                .inspect_err(|e| {
                    let path = "<hooq_root>/tests/special/inert_attr_setting/result_types.rs";
                    let line = 76usize;
                    let col = 13usize;
                    let expr = "return enresult(());";
                    {
                        ::std::io::_eprint(
                            format_args!(
                                "[{0}:{1}:{2}] {3:?}\n    {4}\n", path, line, col, e, expr
                            ),
                        );
                    };
                });
        }
        if enresult(false)
            .inspect_err(|e| {
                let path = "<hooq_root>/tests/special/inert_attr_setting/result_types.rs";
                let line = 79usize;
                let col = 27usize;
                let expr = "enresult(false)?";
                {
                    ::std::io::_eprint(
                        format_args!(
                            "[{0}:{1}:{2}] {3:?}\n    {4}\n", path, line, col, e, expr
                        ),
                    );
                };
            })?
        {
            return Err(())
                .inspect_err(|e| {
                    let path = "<hooq_root>/tests/special/inert_attr_setting/result_types.rs";
                    let line = 80usize;
                    let col = 13usize;
                    let expr = "return Err(());";
                    {
                        ::std::io::_eprint(
                            format_args!(
                                "[{0}:{1}:{2}] {3:?}\n    {4}\n", path, line, col, e, expr
                            ),
                        );
                    };
                });
        }
        if enresult(false)
            .inspect_err(|e| {
                let path = "<hooq_root>/tests/special/inert_attr_setting/result_types.rs";
                let line = 83usize;
                let col = 27usize;
                let expr = "enresult(false)?";
                {
                    ::std::io::_eprint(
                        format_args!(
                            "[{0}:{1}:{2}] {3:?}\n    {4}\n", path, line, col, e, expr
                        ),
                    );
                };
            })?
        {
            return Ok(())
                .inspect_err(|e| {
                    let path = "<hooq_root>/tests/special/inert_attr_setting/result_types.rs";
                    let line = 84usize;
                    let col = 13usize;
                    let expr = "return Ok(());";
                    {
                        ::std::io::_eprint(
                            format_args!(
                                "[{0}:{1}:{2}] {3:?}\n    {4}\n", path, line, col, e, expr
                            ),
                        );
                    };
                });
        }
        Ok(())
            .inspect_err(|e| {
                let path = "<hooq_root>/tests/special/inert_attr_setting/result_types.rs";
                let line = 87usize;
                let col = 9usize;
                let expr = "Ok(())";
                {
                    ::std::io::_eprint(
                        format_args!(
                            "[{0}:{1}:{2}] {3:?}\n    {4}\n", path, line, col, e, expr
                        ),
                    );
                };
            })
    }
    pub fn other_fn_4_2() -> Either {
        if enresult(false)
            .inspect_err(|e| {
                let path = "<hooq_root>/tests/special/inert_attr_setting/result_types.rs";
                let line = 92usize;
                let col = 27usize;
                let expr = "enresult(false)?";
                {
                    ::std::io::_eprint(
                        format_args!(
                            "[{0}:{1}:{2}] {3:?}\n    {4}\n", path, line, col, e, expr
                        ),
                    );
                };
            })?
        {
            return enresult(())
                .inspect_err(|e| {
                    let path = "<hooq_root>/tests/special/inert_attr_setting/result_types.rs";
                    let line = 93usize;
                    let col = 13usize;
                    let expr = "return enresult(());";
                    {
                        ::std::io::_eprint(
                            format_args!(
                                "[{0}:{1}:{2}] {3:?}\n    {4}\n", path, line, col, e, expr
                            ),
                        );
                    };
                });
        }
        if enresult(false)
            .inspect_err(|e| {
                let path = "<hooq_root>/tests/special/inert_attr_setting/result_types.rs";
                let line = 96usize;
                let col = 27usize;
                let expr = "enresult(false)?";
                {
                    ::std::io::_eprint(
                        format_args!(
                            "[{0}:{1}:{2}] {3:?}\n    {4}\n", path, line, col, e, expr
                        ),
                    );
                };
            })?
        {
            return Err(())
                .inspect_err(|e| {
                    let path = "<hooq_root>/tests/special/inert_attr_setting/result_types.rs";
                    let line = 97usize;
                    let col = 13usize;
                    let expr = "return Err(());";
                    {
                        ::std::io::_eprint(
                            format_args!(
                                "[{0}:{1}:{2}] {3:?}\n    {4}\n", path, line, col, e, expr
                            ),
                        );
                    };
                });
        }
        if enresult(false)
            .inspect_err(|e| {
                let path = "<hooq_root>/tests/special/inert_attr_setting/result_types.rs";
                let line = 100usize;
                let col = 27usize;
                let expr = "enresult(false)?";
                {
                    ::std::io::_eprint(
                        format_args!(
                            "[{0}:{1}:{2}] {3:?}\n    {4}\n", path, line, col, e, expr
                        ),
                    );
                };
            })?
        {
            return Ok(());
        }
        Ok(())
    }
}
