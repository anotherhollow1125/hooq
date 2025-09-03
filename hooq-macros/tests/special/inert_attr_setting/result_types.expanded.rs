use hooq_macros::hooq;
mod funcs {
    type Either = Result<(), ()>;
    type NotTarget = Result<(), ()>;
    fn enresult<T>(val: T) -> Result<T, ()> {
        Ok(val)
            .inspect_err(|e| {
                let path = "<hooq_root>/tests/special/inert_attr_setting/result_types.rs";
                let line = 11usize;
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
                let line = 15usize;
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
                    let line = 16usize;
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
                let line = 19usize;
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
                let line = 23usize;
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
                    let line = 24usize;
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
                let line = 27usize;
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
                let line = 31usize;
                {
                    ::std::io::_eprint(
                        format_args!("[{0}:L{1}] {2:?}\n", path, line, e),
                    );
                };
            })?
        {
            return enresult(());
        }
        Ok(())
    }
    pub fn other_fn_2() -> NotTarget {
        if enresult(false)
            .inspect_err(|e| {
                let path = "<hooq_root>/tests/special/inert_attr_setting/result_types.rs";
                let line = 44usize;
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
                let line = 48usize;
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
                    let line = 49usize;
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
                let line = 52usize;
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
                let line = 56usize;
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
                let line = 60usize;
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
            .inspect_err(|e| {
                let path = "<hooq_root>/tests/special/inert_attr_setting/result_types.rs";
                let line = 66usize;
                {
                    ::std::io::_eprint(
                        format_args!("[{0}:L{1}] {2:?}\n", path, line, e),
                    );
                };
            })
    }
}
