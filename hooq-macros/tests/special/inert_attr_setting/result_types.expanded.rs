use hooq_macros::hooq;
mod funcs {
    type Either = Result<(), ()>;
    type NotTarget = Result<(), ()>;
    fn enresult<T>(val: T) -> Result<T, ()> {
        Ok(val)
            .inspect_err(|e| {
                let path = "<hooq_root>/tests/special/inert_attr_setting/result_types.rs";
                let line = 12usize;
                {
                    ::std::io::_eprint(
                        format_args!("[{0}:L{1}] {2:?}\n", path, line, e),
                    );
                };
            })
    }
    pub fn result_fn() -> Result<(), ()> {
        if enresult(false)
            .inspect_err(|e| {
                let path = "<hooq_root>/tests/special/inert_attr_setting/result_types.rs";
                let line = 16usize;
                {
                    ::std::io::_eprint(
                        format_args!("[{0}:L{1}] {2:?}\n", path, line, e),
                    );
                };
            })?
        {
            return enresult(())
                .inspect_err(|e| {
                    let path = "<hooq_root>/tests/special/inert_attr_setting/result_types.rs";
                    let line = 17usize;
                    {
                        ::std::io::_eprint(
                            format_args!("[{0}:L{1}] {2:?}\n", path, line, e),
                        );
                    };
                });
        }
        enresult(())
            .inspect_err(|e| {
                let path = "<hooq_root>/tests/special/inert_attr_setting/result_types.rs";
                let line = 20usize;
                {
                    ::std::io::_eprint(
                        format_args!("[{0}:L{1}] {2:?}\n", path, line, e),
                    );
                };
            })
    }
    pub fn either_fn() -> Either {
        if enresult(false)
            .inspect_err(|e| {
                let path = "<hooq_root>/tests/special/inert_attr_setting/result_types.rs";
                let line = 24usize;
                {
                    ::std::io::_eprint(
                        format_args!("[{0}:L{1}] {2:?}\n", path, line, e),
                    );
                };
            })?
        {
            return enresult(())
                .inspect_err(|e| {
                    let path = "<hooq_root>/tests/special/inert_attr_setting/result_types.rs";
                    let line = 25usize;
                    {
                        ::std::io::_eprint(
                            format_args!("[{0}:L{1}] {2:?}\n", path, line, e),
                        );
                    };
                });
        }
        enresult(())
            .inspect_err(|e| {
                let path = "<hooq_root>/tests/special/inert_attr_setting/result_types.rs";
                let line = 28usize;
                {
                    ::std::io::_eprint(
                        format_args!("[{0}:L{1}] {2:?}\n", path, line, e),
                    );
                };
            })
    }
    pub fn other_fn_1() -> NotTarget {
        if enresult(false)
            .inspect_err(|e| {
                let path = "<hooq_root>/tests/special/inert_attr_setting/result_types.rs";
                let line = 32usize;
                {
                    ::std::io::_eprint(
                        format_args!("[{0}:L{1}] {2:?}\n", path, line, e),
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
                {
                    ::std::io::_eprint(
                        format_args!("[{0}:L{1}] {2:?}\n", path, line, e),
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
                {
                    ::std::io::_eprint(
                        format_args!("[{0}:L{1}] {2:?}\n", path, line, e),
                    );
                };
            })?
        {
            return Ok(())
                .inspect_err(|e| {
                    let path = "<hooq_root>/tests/special/inert_attr_setting/result_types.rs";
                    let line = 50usize;
                    {
                        ::std::io::_eprint(
                            format_args!("[{0}:L{1}] {2:?}\n", path, line, e),
                        );
                    };
                });
        }
        Ok(())
            .inspect_err(|e| {
                let path = "<hooq_root>/tests/special/inert_attr_setting/result_types.rs";
                let line = 53usize;
                {
                    ::std::io::_eprint(
                        format_args!("[{0}:L{1}] {2:?}\n", path, line, e),
                    );
                };
            })
    }
    pub fn other_fn_3() -> NotTarget {
        if enresult(false)
            .inspect_err(|e| {
                let path = "<hooq_root>/tests/special/inert_attr_setting/result_types.rs";
                let line = 57usize;
                {
                    ::std::io::_eprint(
                        format_args!("[{0}:L{1}] {2:?}\n", path, line, e),
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
                {
                    ::std::io::_eprint(
                        format_args!("[{0}:L{1}] {2:?}\n", path, line, e),
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
                {
                    ::std::io::_eprint(
                        format_args!("[{0}:L{1}] {2:?}\n", path, line, e),
                    );
                };
            })
    }
    pub fn other_fn_4_1() -> Either {
        if enresult(false)
            .inspect_err(|e| {
                let path = "<hooq_root>/tests/special/inert_attr_setting/result_types.rs";
                let line = 75usize;
                {
                    ::std::io::_eprint(
                        format_args!("[{0}:L{1}] {2:?}\n", path, line, e),
                    );
                };
            })?
        {
            return enresult(())
                .inspect_err(|e| {
                    let path = "<hooq_root>/tests/special/inert_attr_setting/result_types.rs";
                    let line = 76usize;
                    {
                        ::std::io::_eprint(
                            format_args!("[{0}:L{1}] {2:?}\n", path, line, e),
                        );
                    };
                });
        }
        if enresult(false)
            .inspect_err(|e| {
                let path = "<hooq_root>/tests/special/inert_attr_setting/result_types.rs";
                let line = 79usize;
                {
                    ::std::io::_eprint(
                        format_args!("[{0}:L{1}] {2:?}\n", path, line, e),
                    );
                };
            })?
        {
            return Err(())
                .inspect_err(|e| {
                    let path = "<hooq_root>/tests/special/inert_attr_setting/result_types.rs";
                    let line = 80usize;
                    {
                        ::std::io::_eprint(
                            format_args!("[{0}:L{1}] {2:?}\n", path, line, e),
                        );
                    };
                });
        }
        if enresult(false)
            .inspect_err(|e| {
                let path = "<hooq_root>/tests/special/inert_attr_setting/result_types.rs";
                let line = 83usize;
                {
                    ::std::io::_eprint(
                        format_args!("[{0}:L{1}] {2:?}\n", path, line, e),
                    );
                };
            })?
        {
            return Ok(())
                .inspect_err(|e| {
                    let path = "<hooq_root>/tests/special/inert_attr_setting/result_types.rs";
                    let line = 84usize;
                    {
                        ::std::io::_eprint(
                            format_args!("[{0}:L{1}] {2:?}\n", path, line, e),
                        );
                    };
                });
        }
        Ok(())
            .inspect_err(|e| {
                let path = "<hooq_root>/tests/special/inert_attr_setting/result_types.rs";
                let line = 87usize;
                {
                    ::std::io::_eprint(
                        format_args!("[{0}:L{1}] {2:?}\n", path, line, e),
                    );
                };
            })
    }
    pub fn other_fn_4_2() -> Either {
        if enresult(false)
            .inspect_err(|e| {
                let path = "<hooq_root>/tests/special/inert_attr_setting/result_types.rs";
                let line = 92usize;
                {
                    ::std::io::_eprint(
                        format_args!("[{0}:L{1}] {2:?}\n", path, line, e),
                    );
                };
            })?
        {
            return enresult(())
                .inspect_err(|e| {
                    let path = "<hooq_root>/tests/special/inert_attr_setting/result_types.rs";
                    let line = 93usize;
                    {
                        ::std::io::_eprint(
                            format_args!("[{0}:L{1}] {2:?}\n", path, line, e),
                        );
                    };
                });
        }
        if enresult(false)
            .inspect_err(|e| {
                let path = "<hooq_root>/tests/special/inert_attr_setting/result_types.rs";
                let line = 96usize;
                {
                    ::std::io::_eprint(
                        format_args!("[{0}:L{1}] {2:?}\n", path, line, e),
                    );
                };
            })?
        {
            return Err(())
                .inspect_err(|e| {
                    let path = "<hooq_root>/tests/special/inert_attr_setting/result_types.rs";
                    let line = 97usize;
                    {
                        ::std::io::_eprint(
                            format_args!("[{0}:L{1}] {2:?}\n", path, line, e),
                        );
                    };
                });
        }
        if enresult(false)
            .inspect_err(|e| {
                let path = "<hooq_root>/tests/special/inert_attr_setting/result_types.rs";
                let line = 100usize;
                {
                    ::std::io::_eprint(
                        format_args!("[{0}:L{1}] {2:?}\n", path, line, e),
                    );
                };
            })?
        {
            return Ok(());
        }
        Ok(())
    }
}
