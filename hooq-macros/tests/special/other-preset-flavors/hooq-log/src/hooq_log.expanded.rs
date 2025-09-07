use hooq::hooq;
pub fn a() -> Result<(), &'static str> {
    Err("error!")
        .inspect_err(|e| {
            let line = 5usize;
            let expr_str = "Err(\"error!\")";
            let expr_str = if expr_str.len() > 20 {
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("...{0}", & expr_str[expr_str.len() - 20..]),
                    )
                })
            } else {
                expr_str.to_string()
            };
            {
                {
                    let lvl = ::log::Level::Error;
                    if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                        ::log::__private_api::log(
                            { ::log::__private_api::GlobalLogger },
                            format_args!("(L{0}) {1} from {2}", line, e, expr_str),
                            lvl,
                            &("hooq_log", "hooq_log", ::log::__private_api::loc()),
                            (),
                        );
                    }
                }
            };
        })
}
pub fn b() -> Result<(), &'static str> {
    a()
        .inspect_err(|e| {
            let line = 10usize;
            let expr_str = "a()";
            let expr_str = if expr_str.len() > 20 {
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("...{0}", & expr_str[expr_str.len() - 20..]),
                    )
                })
            } else {
                expr_str.to_string()
            };
            {
                {
                    let lvl = ::log::Level::Error;
                    if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                        ::log::__private_api::log(
                            { ::log::__private_api::GlobalLogger },
                            format_args!("(L{0}) {1} from {2}", line, e, expr_str),
                            lvl,
                            &("hooq_log", "hooq_log", ::log::__private_api::loc()),
                            (),
                        );
                    }
                }
            };
        })?;
    Ok(())
        .inspect_err(|e| {
            let line = 12usize;
            let expr_str = "Ok(())";
            let expr_str = if expr_str.len() > 20 {
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("...{0}", & expr_str[expr_str.len() - 20..]),
                    )
                })
            } else {
                expr_str.to_string()
            };
            {
                {
                    let lvl = ::log::Level::Error;
                    if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                        ::log::__private_api::log(
                            { ::log::__private_api::GlobalLogger },
                            format_args!("(L{0}) {1} from {2}", line, e, expr_str),
                            lvl,
                            &("hooq_log", "hooq_log", ::log::__private_api::loc()),
                            (),
                        );
                    }
                }
            };
        })
}
pub fn c() -> Result<(), &'static str> {
    b()
        .inspect_err(|e| {
            let line = 17usize;
            let expr_str = "b()";
            let expr_str = if expr_str.len() > 20 {
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("...{0}", & expr_str[expr_str.len() - 20..]),
                    )
                })
            } else {
                expr_str.to_string()
            };
            {
                {
                    let lvl = ::log::Level::Error;
                    if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                        ::log::__private_api::log(
                            { ::log::__private_api::GlobalLogger },
                            format_args!("(L{0}) {1} from {2}", line, e, expr_str),
                            lvl,
                            &("hooq_log", "hooq_log", ::log::__private_api::loc()),
                            (),
                        );
                    }
                }
            };
        })?;
    Ok(())
        .inspect_err(|e| {
            let line = 19usize;
            let expr_str = "Ok(())";
            let expr_str = if expr_str.len() > 20 {
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("...{0}", & expr_str[expr_str.len() - 20..]),
                    )
                })
            } else {
                expr_str.to_string()
            };
            {
                {
                    let lvl = ::log::Level::Error;
                    if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                        ::log::__private_api::log(
                            { ::log::__private_api::GlobalLogger },
                            format_args!("(L{0}) {1} from {2}", line, e, expr_str),
                            lvl,
                            &("hooq_log", "hooq_log", ::log::__private_api::loc()),
                            (),
                        );
                    }
                }
            };
        })
}
