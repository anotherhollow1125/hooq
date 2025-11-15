use hooq::hooq;
pub fn a() -> Result<(), &'static str> {
    Err("error!")
        .inspect_err(|e| {
            let path = "<hooq_root>/src/hooq_log.rs";
            let line = 5usize;
            let col = 5usize;
            let expr = "   5|     Err(\"error!\")\n    |";
            {
                {
                    let lvl = ::log::Level::Error;
                    if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                        ::log::__private_api::log(
                            { ::log::__private_api::GlobalLogger },
                            format_args!(
                                "({0}:{1}:{2}) {3}\n{4}", path, line, col, e, expr
                            ),
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
            let path = "<hooq_root>/src/hooq_log.rs";
            let line = 10usize;
            let col = 8usize;
            let expr = "  10|     a()?\n    |";
            {
                {
                    let lvl = ::log::Level::Error;
                    if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                        ::log::__private_api::log(
                            { ::log::__private_api::GlobalLogger },
                            format_args!(
                                "({0}:{1}:{2}) {3}\n{4}", path, line, col, e, expr
                            ),
                            lvl,
                            &("hooq_log", "hooq_log", ::log::__private_api::loc()),
                            (),
                        );
                    }
                }
            };
        })?;
    Err("err")
        .inspect_err(|e| {
            let path = "<hooq_root>/src/hooq_log.rs";
            let line = 12usize;
            let col = 5usize;
            let expr = "  12|     Err(\"err\")\n    |";
            {
                {
                    let lvl = ::log::Level::Error;
                    if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                        ::log::__private_api::log(
                            { ::log::__private_api::GlobalLogger },
                            format_args!(
                                "({0}:{1}:{2}) {3}\n{4}", path, line, col, e, expr
                            ),
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
            let path = "<hooq_root>/src/hooq_log.rs";
            let line = 17usize;
            let col = 8usize;
            let expr = "  17|     b()?\n    |";
            {
                {
                    let lvl = ::log::Level::Error;
                    if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                        ::log::__private_api::log(
                            { ::log::__private_api::GlobalLogger },
                            format_args!(
                                "({0}:{1}:{2}) {3}\n{4}", path, line, col, e, expr
                            ),
                            lvl,
                            &("hooq_log", "hooq_log", ::log::__private_api::loc()),
                            (),
                        );
                    }
                }
            };
        })?;
    Err("err")
        .inspect_err(|e| {
            let path = "<hooq_root>/src/hooq_log.rs";
            let line = 19usize;
            let col = 5usize;
            let expr = "  19|     Err(\"err\")\n    |";
            {
                {
                    let lvl = ::log::Level::Error;
                    if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                        ::log::__private_api::log(
                            { ::log::__private_api::GlobalLogger },
                            format_args!(
                                "({0}:{1}:{2}) {3}\n{4}", path, line, col, e, expr
                            ),
                            lvl,
                            &("hooq_log", "hooq_log", ::log::__private_api::loc()),
                            (),
                        );
                    }
                }
            };
        })
}
