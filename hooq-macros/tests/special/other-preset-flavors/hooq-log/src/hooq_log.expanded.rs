use hooq::hooq;
pub fn a() -> Result<(), &'static str> {
    Err("error!")
        .inspect_err(|e| {
            let path = "<hooq_root>/src/hooq_log.rs";
            let line = 5usize;
            let col = 5usize;
            let expr = "   5>    Err(\"error!\")\n    |";
            let level = ::log::Level::Error;
            {
                {
                    let lvl = level;
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
            let expr = "  10>    a()?\n    |";
            let level = ::log::Level::Error;
            {
                {
                    let lvl = level;
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
            let expr = "  12>    Err(\"err\")\n    |";
            let level = ::log::Level::Error;
            {
                {
                    let lvl = level;
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
            let expr = "  17>    b()?\n    |";
            let level = ::log::Level::Warn;
            {
                {
                    let lvl = level;
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
            let expr = "  19>    Err(\"err\")\n    |";
            let level = ::log::Level::Warn;
            {
                {
                    let lvl = level;
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
pub fn d() -> Result<(), &'static str> {
    c()
        .inspect_err(|e| {
            let path = "<hooq_root>/src/hooq_log.rs";
            let line = 24usize;
            let col = 8usize;
            let expr = "  24>    c()?\n    |";
            let level = ::log::Level::Info;
            {
                {
                    let lvl = level;
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
            let line = 26usize;
            let col = 5usize;
            let expr = "  26>    Err(\"err\")\n    |";
            let level = ::log::Level::Info;
            {
                {
                    let lvl = level;
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
pub fn e() -> Result<(), &'static str> {
    d()
        .inspect_err(|e| {
            let path = "<hooq_root>/src/hooq_log.rs";
            let line = 31usize;
            let col = 8usize;
            let expr = "  31>    d()?\n    |";
            let level = ::log::Level::Debug;
            {
                {
                    let lvl = level;
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
            let line = 33usize;
            let col = 5usize;
            let expr = "  33>    Err(\"err\")\n    |";
            let level = ::log::Level::Debug;
            {
                {
                    let lvl = level;
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
pub fn f() -> Result<(), &'static str> {
    e()
        .inspect_err(|e| {
            let path = "<hooq_root>/src/hooq_log.rs";
            let line = 38usize;
            let col = 8usize;
            let expr = "  38>    e()?\n    |";
            let level = ::log::Level::Trace;
            {
                {
                    let lvl = level;
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
            let line = 40usize;
            let col = 5usize;
            let expr = "  40>    Err(\"err\")\n    |";
            let level = ::log::Level::Trace;
            {
                {
                    let lvl = level;
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
