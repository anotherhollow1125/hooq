use hooq_macros::hooq;
fn hoge() -> Result<(), ()> {
    Ok(())
}
fn option_fn_1(flag: bool, flagflag: bool) -> Option<()> {
    hoge()
        .ok()
        .inspect(|_| {
            {
                ::std::io::_eprint(
                    format_args!(
                        "{0:?}\n", ::hooq::HooqMeta { line : 23usize, column : 16usize,
                        path :
                        "<hooq_root>/tests/special/inert_attr_setting/option.rs",
                        file : "option.rs", expr_str : "hoge().ok() ?", count : "1st ?",
                        bindings : ::std::collections::HashMap::from([]), }
                    ),
                );
            };
        })?;
    if !flag {
        return None
            .inspect(|_| {
                {
                    ::std::io::_eprint(
                        format_args!(
                            "{0:?}\n", ::hooq::HooqMeta { line : 26usize, column :
                            9usize, path :
                            "<hooq_root>/tests/special/inert_attr_setting/option.rs",
                            file : "option.rs", expr_str : "return None", count :
                            "1st return", bindings :
                            ::std::collections::HashMap::from([]), }
                        ),
                    );
                };
            });
    }
    let _ = || {
        if flag {
            return Some(())
                .inspect(|_| {
                    {
                        ::std::io::_eprint(
                            format_args!(
                                "{0:?}\n", ::hooq::HooqMeta { line : 32usize, column :
                                13usize, path :
                                "<hooq_root>/tests/special/inert_attr_setting/option.rs",
                                file : "option.rs", expr_str : "return Some(())", count :
                                "2nd return", bindings :
                                ::std::collections::HashMap::from([]), }
                            ),
                        );
                    };
                });
        }
        if flagflag {
            return None;
        }
        None
    };
    Some(())
        .inspect(|_| {
            {
                ::std::io::_eprint(
                    format_args!(
                        "{0:?}\n", ::hooq::HooqMeta { line : 42usize, column : 5usize,
                        path :
                        "<hooq_root>/tests/special/inert_attr_setting/option.rs",
                        file : "option.rs", expr_str : "Some(())", count :
                        "1st tail expr", bindings :
                        ::std::collections::HashMap::from([]), }
                    ),
                );
            };
        })
}
fn option_fn_2(flag: bool, flagflag: bool) -> Option<()> {
    hoge()
        .ok()
        .inspect(|_| {
            {
                ::std::io::_eprint(
                    format_args!(
                        "{0:?}\n", ::hooq::HooqMeta { line : 52usize, column : 16usize,
                        path :
                        "<hooq_root>/tests/special/inert_attr_setting/option.rs",
                        file : "option.rs", expr_str : "hoge().ok() ?", count : "1st ?",
                        bindings : ::std::collections::HashMap::from([]), }
                    ),
                );
            };
        })?;
    if !flag {
        return None
            .inspect(|_| {
                {
                    ::std::io::_eprint(
                        format_args!(
                            "{0:?}\n", ::hooq::HooqMeta { line : 56usize, column :
                            9usize, path :
                            "<hooq_root>/tests/special/inert_attr_setting/option.rs",
                            file : "option.rs", expr_str : "return None", count :
                            "1st return", bindings :
                            ::std::collections::HashMap::from([]), }
                        ),
                    );
                };
            });
    }
    let _ = || {
        if flag {
            return Some(())
                .inspect(|_| {
                    {
                        ::std::io::_eprint(
                            format_args!(
                                "{0:?}\n", ::hooq::HooqMeta { line : 62usize, column :
                                13usize, path :
                                "<hooq_root>/tests/special/inert_attr_setting/option.rs",
                                file : "option.rs", expr_str : "return Some(())", count :
                                "2nd return", bindings :
                                ::std::collections::HashMap::from([]), }
                            ),
                        );
                    };
                });
        }
        if flagflag {
            return None;
        }
        None
    };
    None.inspect(|_| {
        {
            ::std::io::_eprint(
                format_args!(
                    "{0:?}\n", ::hooq::HooqMeta { line : 72usize, column : 5usize, path :
                    "<hooq_root>/tests/special/inert_attr_setting/option.rs",
                    file : "option.rs", expr_str : "None", count : "1st tail expr",
                    bindings : ::std::collections::HashMap::from([]), }
                ),
            );
        };
    })
}
