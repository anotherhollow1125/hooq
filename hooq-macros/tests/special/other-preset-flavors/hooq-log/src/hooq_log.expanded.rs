use hooq::hooq;
pub fn a() -> Result<(), &'static str> {
    Err("error!")
        .inspect_err(|e| {
            let line = 5usize;
            {
                {
                    let lvl = ::log::Level::Error;
                    if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                        ::log::__private_api::log(
                            { ::log::__private_api::GlobalLogger },
                            format_args!("(L{0}) {1}", line, e),
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
            {
                {
                    let lvl = ::log::Level::Error;
                    if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                        ::log::__private_api::log(
                            { ::log::__private_api::GlobalLogger },
                            format_args!("(L{0}) {1}", line, e),
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
            {
                {
                    let lvl = ::log::Level::Error;
                    if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                        ::log::__private_api::log(
                            { ::log::__private_api::GlobalLogger },
                            format_args!("(L{0}) {1}", line, e),
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
            {
                {
                    let lvl = ::log::Level::Error;
                    if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                        ::log::__private_api::log(
                            { ::log::__private_api::GlobalLogger },
                            format_args!("(L{0}) {1}", line, e),
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
            {
                {
                    let lvl = ::log::Level::Error;
                    if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                        ::log::__private_api::log(
                            { ::log::__private_api::GlobalLogger },
                            format_args!("(L{0}) {1}", line, e),
                            lvl,
                            &("hooq_log", "hooq_log", ::log::__private_api::loc()),
                            (),
                        );
                    }
                }
            };
        })
}
