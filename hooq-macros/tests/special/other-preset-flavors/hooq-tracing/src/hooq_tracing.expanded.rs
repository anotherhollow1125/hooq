use hooq::hooq;
use tracing::instrument;
pub fn a() -> Result<(), String> {
    {}
    #[allow(clippy::suspicious_else_formatting)]
    {
        let __tracing_attr_span;
        let __tracing_attr_guard;
        if ::tracing::Level::INFO <= ::tracing::level_filters::STATIC_MAX_LEVEL
            && ::tracing::Level::INFO <= ::tracing::level_filters::LevelFilter::current()
            || { false }
        {
            __tracing_attr_span = {
                use ::tracing::__macro_support::Callsite as _;
                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "a",
                            "hooq_tracing",
                            ::tracing::Level::INFO,
                            ::tracing_core::__macro_support::Option::Some(
                                "<hooq_root>/src/hooq_tracing.rs",
                            ),
                            ::tracing_core::__macro_support::Option::Some(5u32),
                            ::tracing_core::__macro_support::Option::Some(
                                "hooq_tracing",
                            ),
                            ::tracing_core::field::FieldSet::new(
                                &[],
                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                            ),
                            ::tracing::metadata::Kind::SPAN,
                        )
                    };
                    ::tracing::callsite::DefaultCallsite::new(&META)
                };
                let mut interest = ::tracing::subscriber::Interest::never();
                if ::tracing::Level::INFO <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::tracing::Level::INFO
                        <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        interest = __CALLSITE.interest();
                        !interest.is_never()
                    }
                    && ::tracing::__macro_support::__is_enabled(
                        __CALLSITE.metadata(),
                        interest,
                    )
                {
                    let meta = __CALLSITE.metadata();
                    ::tracing::Span::new(meta, &{ meta.fields().value_set_all(&[]) })
                } else {
                    let span = ::tracing::__macro_support::__disabled_span(
                        __CALLSITE.metadata(),
                    );
                    {};
                    span
                }
            };
            __tracing_attr_guard = __tracing_attr_span.enter();
        }
        #[warn(clippy::suspicious_else_formatting)]
        {
            #[allow(
                unknown_lints,
                unreachable_code,
                clippy::diverging_sub_expression,
                clippy::empty_loop,
                clippy::let_unit_value,
                clippy::let_with_type_underscore,
                clippy::needless_return,
                clippy::unreachable
            )]
            if false {
                let __tracing_attr_fake_return: Result<(), String> = loop {};
                return __tracing_attr_fake_return;
            }
            {
                Err("error!".to_string())
                    .inspect_err(|e| {
                        let path = "<hooq_root>/src/hooq_tracing.rs";
                        let line = 7usize;
                        let col = 5usize;
                        let expr = "Err(\"error!\".to_string())";
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event <hooq_root>/src/hooq_tracing.rs:4",
                                        "hooq_tracing",
                                        ::tracing::Level::ERROR,
                                        ::tracing_core::__macro_support::Option::Some(
                                            "<hooq_root>/src/hooq_tracing.rs",
                                        ),
                                        ::tracing_core::__macro_support::Option::Some(4u32),
                                        ::tracing_core::__macro_support::Option::Some(
                                            "hooq_tracing",
                                        ),
                                        ::tracing_core::field::FieldSet::new(
                                            &[
                                                {
                                                    const NAME: ::tracing::__macro_support::FieldName<
                                                        { ::tracing::__macro_support::FieldName::len("path") },
                                                    > = ::tracing::__macro_support::FieldName::new("path");
                                                    NAME.as_str()
                                                },
                                                {
                                                    const NAME: ::tracing::__macro_support::FieldName<
                                                        { ::tracing::__macro_support::FieldName::len("line") },
                                                    > = ::tracing::__macro_support::FieldName::new("line");
                                                    NAME.as_str()
                                                },
                                                {
                                                    const NAME: ::tracing::__macro_support::FieldName<
                                                        { ::tracing::__macro_support::FieldName::len("col") },
                                                    > = ::tracing::__macro_support::FieldName::new("col");
                                                    NAME.as_str()
                                                },
                                                {
                                                    const NAME: ::tracing::__macro_support::FieldName<
                                                        { ::tracing::__macro_support::FieldName::len("error") },
                                                    > = ::tracing::__macro_support::FieldName::new("error");
                                                    NAME.as_str()
                                                },
                                                {
                                                    const NAME: ::tracing::__macro_support::FieldName<
                                                        { ::tracing::__macro_support::FieldName::len("expr") },
                                                    > = ::tracing::__macro_support::FieldName::new("expr");
                                                    NAME.as_str()
                                                },
                                            ],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::ERROR
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::ERROR
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set_all(
                                            &[
                                                (::tracing::__macro_support::Option::Some(
                                                    &path as &dyn ::tracing::field::Value,
                                                )),
                                                (::tracing::__macro_support::Option::Some(
                                                    &line as &dyn ::tracing::field::Value,
                                                )),
                                                (::tracing::__macro_support::Option::Some(
                                                    &col as &dyn ::tracing::field::Value,
                                                )),
                                                (::tracing::__macro_support::Option::Some(
                                                    &::tracing::field::display(&e)
                                                        as &dyn ::tracing::field::Value,
                                                )),
                                                (::tracing::__macro_support::Option::Some(
                                                    &expr as &dyn ::tracing::field::Value,
                                                )),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                    })
            }
        }
    }
}
pub fn b() -> Result<(), String> {
    {}
    #[allow(clippy::suspicious_else_formatting)]
    {
        let __tracing_attr_span;
        let __tracing_attr_guard;
        if ::tracing::Level::INFO <= ::tracing::level_filters::STATIC_MAX_LEVEL
            && ::tracing::Level::INFO <= ::tracing::level_filters::LevelFilter::current()
            || { false }
        {
            __tracing_attr_span = {
                use ::tracing::__macro_support::Callsite as _;
                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "b",
                            "hooq_tracing",
                            ::tracing::Level::INFO,
                            ::tracing_core::__macro_support::Option::Some(
                                "<hooq_root>/src/hooq_tracing.rs",
                            ),
                            ::tracing_core::__macro_support::Option::Some(11u32),
                            ::tracing_core::__macro_support::Option::Some(
                                "hooq_tracing",
                            ),
                            ::tracing_core::field::FieldSet::new(
                                &[],
                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                            ),
                            ::tracing::metadata::Kind::SPAN,
                        )
                    };
                    ::tracing::callsite::DefaultCallsite::new(&META)
                };
                let mut interest = ::tracing::subscriber::Interest::never();
                if ::tracing::Level::INFO <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::tracing::Level::INFO
                        <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        interest = __CALLSITE.interest();
                        !interest.is_never()
                    }
                    && ::tracing::__macro_support::__is_enabled(
                        __CALLSITE.metadata(),
                        interest,
                    )
                {
                    let meta = __CALLSITE.metadata();
                    ::tracing::Span::new(meta, &{ meta.fields().value_set_all(&[]) })
                } else {
                    let span = ::tracing::__macro_support::__disabled_span(
                        __CALLSITE.metadata(),
                    );
                    {};
                    span
                }
            };
            __tracing_attr_guard = __tracing_attr_span.enter();
        }
        #[warn(clippy::suspicious_else_formatting)]
        {
            #[allow(
                unknown_lints,
                unreachable_code,
                clippy::diverging_sub_expression,
                clippy::empty_loop,
                clippy::let_unit_value,
                clippy::let_with_type_underscore,
                clippy::needless_return,
                clippy::unreachable
            )]
            if false {
                let __tracing_attr_fake_return: Result<(), String> = loop {};
                return __tracing_attr_fake_return;
            }
            {
                a()
                    .inspect_err(|e| {
                        let path = "<hooq_root>/src/hooq_tracing.rs";
                        let line = 13usize;
                        let col = 8usize;
                        let expr = "a()?";
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event <hooq_root>/src/hooq_tracing.rs:10",
                                        "hooq_tracing",
                                        ::tracing::Level::ERROR,
                                        ::tracing_core::__macro_support::Option::Some(
                                            "<hooq_root>/src/hooq_tracing.rs",
                                        ),
                                        ::tracing_core::__macro_support::Option::Some(10u32),
                                        ::tracing_core::__macro_support::Option::Some(
                                            "hooq_tracing",
                                        ),
                                        ::tracing_core::field::FieldSet::new(
                                            &[
                                                {
                                                    const NAME: ::tracing::__macro_support::FieldName<
                                                        { ::tracing::__macro_support::FieldName::len("path") },
                                                    > = ::tracing::__macro_support::FieldName::new("path");
                                                    NAME.as_str()
                                                },
                                                {
                                                    const NAME: ::tracing::__macro_support::FieldName<
                                                        { ::tracing::__macro_support::FieldName::len("line") },
                                                    > = ::tracing::__macro_support::FieldName::new("line");
                                                    NAME.as_str()
                                                },
                                                {
                                                    const NAME: ::tracing::__macro_support::FieldName<
                                                        { ::tracing::__macro_support::FieldName::len("col") },
                                                    > = ::tracing::__macro_support::FieldName::new("col");
                                                    NAME.as_str()
                                                },
                                                {
                                                    const NAME: ::tracing::__macro_support::FieldName<
                                                        { ::tracing::__macro_support::FieldName::len("error") },
                                                    > = ::tracing::__macro_support::FieldName::new("error");
                                                    NAME.as_str()
                                                },
                                                {
                                                    const NAME: ::tracing::__macro_support::FieldName<
                                                        { ::tracing::__macro_support::FieldName::len("expr") },
                                                    > = ::tracing::__macro_support::FieldName::new("expr");
                                                    NAME.as_str()
                                                },
                                            ],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::ERROR
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::ERROR
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set_all(
                                            &[
                                                (::tracing::__macro_support::Option::Some(
                                                    &path as &dyn ::tracing::field::Value,
                                                )),
                                                (::tracing::__macro_support::Option::Some(
                                                    &line as &dyn ::tracing::field::Value,
                                                )),
                                                (::tracing::__macro_support::Option::Some(
                                                    &col as &dyn ::tracing::field::Value,
                                                )),
                                                (::tracing::__macro_support::Option::Some(
                                                    &::tracing::field::display(&e)
                                                        as &dyn ::tracing::field::Value,
                                                )),
                                                (::tracing::__macro_support::Option::Some(
                                                    &expr as &dyn ::tracing::field::Value,
                                                )),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                    })?;
                Err("error!".to_string())
                    .inspect_err(|e| {
                        let path = "<hooq_root>/src/hooq_tracing.rs";
                        let line = 15usize;
                        let col = 5usize;
                        let expr = "Err(\"error!\".to_string())";
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event <hooq_root>/src/hooq_tracing.rs:10",
                                        "hooq_tracing",
                                        ::tracing::Level::ERROR,
                                        ::tracing_core::__macro_support::Option::Some(
                                            "<hooq_root>/src/hooq_tracing.rs",
                                        ),
                                        ::tracing_core::__macro_support::Option::Some(10u32),
                                        ::tracing_core::__macro_support::Option::Some(
                                            "hooq_tracing",
                                        ),
                                        ::tracing_core::field::FieldSet::new(
                                            &[
                                                {
                                                    const NAME: ::tracing::__macro_support::FieldName<
                                                        { ::tracing::__macro_support::FieldName::len("path") },
                                                    > = ::tracing::__macro_support::FieldName::new("path");
                                                    NAME.as_str()
                                                },
                                                {
                                                    const NAME: ::tracing::__macro_support::FieldName<
                                                        { ::tracing::__macro_support::FieldName::len("line") },
                                                    > = ::tracing::__macro_support::FieldName::new("line");
                                                    NAME.as_str()
                                                },
                                                {
                                                    const NAME: ::tracing::__macro_support::FieldName<
                                                        { ::tracing::__macro_support::FieldName::len("col") },
                                                    > = ::tracing::__macro_support::FieldName::new("col");
                                                    NAME.as_str()
                                                },
                                                {
                                                    const NAME: ::tracing::__macro_support::FieldName<
                                                        { ::tracing::__macro_support::FieldName::len("error") },
                                                    > = ::tracing::__macro_support::FieldName::new("error");
                                                    NAME.as_str()
                                                },
                                                {
                                                    const NAME: ::tracing::__macro_support::FieldName<
                                                        { ::tracing::__macro_support::FieldName::len("expr") },
                                                    > = ::tracing::__macro_support::FieldName::new("expr");
                                                    NAME.as_str()
                                                },
                                            ],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::ERROR
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::ERROR
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set_all(
                                            &[
                                                (::tracing::__macro_support::Option::Some(
                                                    &path as &dyn ::tracing::field::Value,
                                                )),
                                                (::tracing::__macro_support::Option::Some(
                                                    &line as &dyn ::tracing::field::Value,
                                                )),
                                                (::tracing::__macro_support::Option::Some(
                                                    &col as &dyn ::tracing::field::Value,
                                                )),
                                                (::tracing::__macro_support::Option::Some(
                                                    &::tracing::field::display(&e)
                                                        as &dyn ::tracing::field::Value,
                                                )),
                                                (::tracing::__macro_support::Option::Some(
                                                    &expr as &dyn ::tracing::field::Value,
                                                )),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                    })
            }
        }
    }
}
pub fn c() -> Result<(), String> {
    {}
    #[allow(clippy::suspicious_else_formatting)]
    {
        let __tracing_attr_span;
        let __tracing_attr_guard;
        if ::tracing::Level::INFO <= ::tracing::level_filters::STATIC_MAX_LEVEL
            && ::tracing::Level::INFO <= ::tracing::level_filters::LevelFilter::current()
            || { false }
        {
            __tracing_attr_span = {
                use ::tracing::__macro_support::Callsite as _;
                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "c",
                            "hooq_tracing",
                            ::tracing::Level::INFO,
                            ::tracing_core::__macro_support::Option::Some(
                                "<hooq_root>/src/hooq_tracing.rs",
                            ),
                            ::tracing_core::__macro_support::Option::Some(19u32),
                            ::tracing_core::__macro_support::Option::Some(
                                "hooq_tracing",
                            ),
                            ::tracing_core::field::FieldSet::new(
                                &[],
                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                            ),
                            ::tracing::metadata::Kind::SPAN,
                        )
                    };
                    ::tracing::callsite::DefaultCallsite::new(&META)
                };
                let mut interest = ::tracing::subscriber::Interest::never();
                if ::tracing::Level::INFO <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::tracing::Level::INFO
                        <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        interest = __CALLSITE.interest();
                        !interest.is_never()
                    }
                    && ::tracing::__macro_support::__is_enabled(
                        __CALLSITE.metadata(),
                        interest,
                    )
                {
                    let meta = __CALLSITE.metadata();
                    ::tracing::Span::new(meta, &{ meta.fields().value_set_all(&[]) })
                } else {
                    let span = ::tracing::__macro_support::__disabled_span(
                        __CALLSITE.metadata(),
                    );
                    {};
                    span
                }
            };
            __tracing_attr_guard = __tracing_attr_span.enter();
        }
        #[warn(clippy::suspicious_else_formatting)]
        {
            #[allow(
                unknown_lints,
                unreachable_code,
                clippy::diverging_sub_expression,
                clippy::empty_loop,
                clippy::let_unit_value,
                clippy::let_with_type_underscore,
                clippy::needless_return,
                clippy::unreachable
            )]
            if false {
                let __tracing_attr_fake_return: Result<(), String> = loop {};
                return __tracing_attr_fake_return;
            }
            {
                b()
                    .inspect_err(|e| {
                        let path = "<hooq_root>/src/hooq_tracing.rs";
                        let line = 21usize;
                        let col = 8usize;
                        let expr = "b()?";
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event <hooq_root>/src/hooq_tracing.rs:18",
                                        "hooq_tracing",
                                        ::tracing::Level::WARN,
                                        ::tracing_core::__macro_support::Option::Some(
                                            "<hooq_root>/src/hooq_tracing.rs",
                                        ),
                                        ::tracing_core::__macro_support::Option::Some(18u32),
                                        ::tracing_core::__macro_support::Option::Some(
                                            "hooq_tracing",
                                        ),
                                        ::tracing_core::field::FieldSet::new(
                                            &[
                                                {
                                                    const NAME: ::tracing::__macro_support::FieldName<
                                                        { ::tracing::__macro_support::FieldName::len("path") },
                                                    > = ::tracing::__macro_support::FieldName::new("path");
                                                    NAME.as_str()
                                                },
                                                {
                                                    const NAME: ::tracing::__macro_support::FieldName<
                                                        { ::tracing::__macro_support::FieldName::len("line") },
                                                    > = ::tracing::__macro_support::FieldName::new("line");
                                                    NAME.as_str()
                                                },
                                                {
                                                    const NAME: ::tracing::__macro_support::FieldName<
                                                        { ::tracing::__macro_support::FieldName::len("col") },
                                                    > = ::tracing::__macro_support::FieldName::new("col");
                                                    NAME.as_str()
                                                },
                                                {
                                                    const NAME: ::tracing::__macro_support::FieldName<
                                                        { ::tracing::__macro_support::FieldName::len("error") },
                                                    > = ::tracing::__macro_support::FieldName::new("error");
                                                    NAME.as_str()
                                                },
                                                {
                                                    const NAME: ::tracing::__macro_support::FieldName<
                                                        { ::tracing::__macro_support::FieldName::len("expr") },
                                                    > = ::tracing::__macro_support::FieldName::new("expr");
                                                    NAME.as_str()
                                                },
                                            ],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::WARN
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::WARN
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set_all(
                                            &[
                                                (::tracing::__macro_support::Option::Some(
                                                    &path as &dyn ::tracing::field::Value,
                                                )),
                                                (::tracing::__macro_support::Option::Some(
                                                    &line as &dyn ::tracing::field::Value,
                                                )),
                                                (::tracing::__macro_support::Option::Some(
                                                    &col as &dyn ::tracing::field::Value,
                                                )),
                                                (::tracing::__macro_support::Option::Some(
                                                    &::tracing::field::display(&e)
                                                        as &dyn ::tracing::field::Value,
                                                )),
                                                (::tracing::__macro_support::Option::Some(
                                                    &expr as &dyn ::tracing::field::Value,
                                                )),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                    })?;
                Err("error!".to_string())
                    .inspect_err(|e| {
                        let path = "<hooq_root>/src/hooq_tracing.rs";
                        let line = 23usize;
                        let col = 5usize;
                        let expr = "Err(\"error!\".to_string())";
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event <hooq_root>/src/hooq_tracing.rs:18",
                                        "hooq_tracing",
                                        ::tracing::Level::WARN,
                                        ::tracing_core::__macro_support::Option::Some(
                                            "<hooq_root>/src/hooq_tracing.rs",
                                        ),
                                        ::tracing_core::__macro_support::Option::Some(18u32),
                                        ::tracing_core::__macro_support::Option::Some(
                                            "hooq_tracing",
                                        ),
                                        ::tracing_core::field::FieldSet::new(
                                            &[
                                                {
                                                    const NAME: ::tracing::__macro_support::FieldName<
                                                        { ::tracing::__macro_support::FieldName::len("path") },
                                                    > = ::tracing::__macro_support::FieldName::new("path");
                                                    NAME.as_str()
                                                },
                                                {
                                                    const NAME: ::tracing::__macro_support::FieldName<
                                                        { ::tracing::__macro_support::FieldName::len("line") },
                                                    > = ::tracing::__macro_support::FieldName::new("line");
                                                    NAME.as_str()
                                                },
                                                {
                                                    const NAME: ::tracing::__macro_support::FieldName<
                                                        { ::tracing::__macro_support::FieldName::len("col") },
                                                    > = ::tracing::__macro_support::FieldName::new("col");
                                                    NAME.as_str()
                                                },
                                                {
                                                    const NAME: ::tracing::__macro_support::FieldName<
                                                        { ::tracing::__macro_support::FieldName::len("error") },
                                                    > = ::tracing::__macro_support::FieldName::new("error");
                                                    NAME.as_str()
                                                },
                                                {
                                                    const NAME: ::tracing::__macro_support::FieldName<
                                                        { ::tracing::__macro_support::FieldName::len("expr") },
                                                    > = ::tracing::__macro_support::FieldName::new("expr");
                                                    NAME.as_str()
                                                },
                                            ],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::WARN
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::WARN
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set_all(
                                            &[
                                                (::tracing::__macro_support::Option::Some(
                                                    &path as &dyn ::tracing::field::Value,
                                                )),
                                                (::tracing::__macro_support::Option::Some(
                                                    &line as &dyn ::tracing::field::Value,
                                                )),
                                                (::tracing::__macro_support::Option::Some(
                                                    &col as &dyn ::tracing::field::Value,
                                                )),
                                                (::tracing::__macro_support::Option::Some(
                                                    &::tracing::field::display(&e)
                                                        as &dyn ::tracing::field::Value,
                                                )),
                                                (::tracing::__macro_support::Option::Some(
                                                    &expr as &dyn ::tracing::field::Value,
                                                )),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                    })
            }
        }
    }
}
pub fn d() -> Result<(), String> {
    {}
    #[allow(clippy::suspicious_else_formatting)]
    {
        let __tracing_attr_span;
        let __tracing_attr_guard;
        if ::tracing::Level::INFO <= ::tracing::level_filters::STATIC_MAX_LEVEL
            && ::tracing::Level::INFO <= ::tracing::level_filters::LevelFilter::current()
            || { false }
        {
            __tracing_attr_span = {
                use ::tracing::__macro_support::Callsite as _;
                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "d",
                            "hooq_tracing",
                            ::tracing::Level::INFO,
                            ::tracing_core::__macro_support::Option::Some(
                                "<hooq_root>/src/hooq_tracing.rs",
                            ),
                            ::tracing_core::__macro_support::Option::Some(27u32),
                            ::tracing_core::__macro_support::Option::Some(
                                "hooq_tracing",
                            ),
                            ::tracing_core::field::FieldSet::new(
                                &[],
                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                            ),
                            ::tracing::metadata::Kind::SPAN,
                        )
                    };
                    ::tracing::callsite::DefaultCallsite::new(&META)
                };
                let mut interest = ::tracing::subscriber::Interest::never();
                if ::tracing::Level::INFO <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::tracing::Level::INFO
                        <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        interest = __CALLSITE.interest();
                        !interest.is_never()
                    }
                    && ::tracing::__macro_support::__is_enabled(
                        __CALLSITE.metadata(),
                        interest,
                    )
                {
                    let meta = __CALLSITE.metadata();
                    ::tracing::Span::new(meta, &{ meta.fields().value_set_all(&[]) })
                } else {
                    let span = ::tracing::__macro_support::__disabled_span(
                        __CALLSITE.metadata(),
                    );
                    {};
                    span
                }
            };
            __tracing_attr_guard = __tracing_attr_span.enter();
        }
        #[warn(clippy::suspicious_else_formatting)]
        {
            #[allow(
                unknown_lints,
                unreachable_code,
                clippy::diverging_sub_expression,
                clippy::empty_loop,
                clippy::let_unit_value,
                clippy::let_with_type_underscore,
                clippy::needless_return,
                clippy::unreachable
            )]
            if false {
                let __tracing_attr_fake_return: Result<(), String> = loop {};
                return __tracing_attr_fake_return;
            }
            {
                c()
                    .inspect_err(|e| {
                        let path = "<hooq_root>/src/hooq_tracing.rs";
                        let line = 29usize;
                        let col = 8usize;
                        let expr = "c()?";
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event <hooq_root>/src/hooq_tracing.rs:26",
                                        "hooq_tracing",
                                        ::tracing::Level::INFO,
                                        ::tracing_core::__macro_support::Option::Some(
                                            "<hooq_root>/src/hooq_tracing.rs",
                                        ),
                                        ::tracing_core::__macro_support::Option::Some(26u32),
                                        ::tracing_core::__macro_support::Option::Some(
                                            "hooq_tracing",
                                        ),
                                        ::tracing_core::field::FieldSet::new(
                                            &[
                                                {
                                                    const NAME: ::tracing::__macro_support::FieldName<
                                                        { ::tracing::__macro_support::FieldName::len("path") },
                                                    > = ::tracing::__macro_support::FieldName::new("path");
                                                    NAME.as_str()
                                                },
                                                {
                                                    const NAME: ::tracing::__macro_support::FieldName<
                                                        { ::tracing::__macro_support::FieldName::len("line") },
                                                    > = ::tracing::__macro_support::FieldName::new("line");
                                                    NAME.as_str()
                                                },
                                                {
                                                    const NAME: ::tracing::__macro_support::FieldName<
                                                        { ::tracing::__macro_support::FieldName::len("col") },
                                                    > = ::tracing::__macro_support::FieldName::new("col");
                                                    NAME.as_str()
                                                },
                                                {
                                                    const NAME: ::tracing::__macro_support::FieldName<
                                                        { ::tracing::__macro_support::FieldName::len("error") },
                                                    > = ::tracing::__macro_support::FieldName::new("error");
                                                    NAME.as_str()
                                                },
                                                {
                                                    const NAME: ::tracing::__macro_support::FieldName<
                                                        { ::tracing::__macro_support::FieldName::len("expr") },
                                                    > = ::tracing::__macro_support::FieldName::new("expr");
                                                    NAME.as_str()
                                                },
                                            ],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::INFO
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::INFO
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set_all(
                                            &[
                                                (::tracing::__macro_support::Option::Some(
                                                    &path as &dyn ::tracing::field::Value,
                                                )),
                                                (::tracing::__macro_support::Option::Some(
                                                    &line as &dyn ::tracing::field::Value,
                                                )),
                                                (::tracing::__macro_support::Option::Some(
                                                    &col as &dyn ::tracing::field::Value,
                                                )),
                                                (::tracing::__macro_support::Option::Some(
                                                    &::tracing::field::display(&e)
                                                        as &dyn ::tracing::field::Value,
                                                )),
                                                (::tracing::__macro_support::Option::Some(
                                                    &expr as &dyn ::tracing::field::Value,
                                                )),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                    })?;
                Err("error!".to_string())
                    .inspect_err(|e| {
                        let path = "<hooq_root>/src/hooq_tracing.rs";
                        let line = 31usize;
                        let col = 5usize;
                        let expr = "Err(\"error!\".to_string())";
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event <hooq_root>/src/hooq_tracing.rs:26",
                                        "hooq_tracing",
                                        ::tracing::Level::INFO,
                                        ::tracing_core::__macro_support::Option::Some(
                                            "<hooq_root>/src/hooq_tracing.rs",
                                        ),
                                        ::tracing_core::__macro_support::Option::Some(26u32),
                                        ::tracing_core::__macro_support::Option::Some(
                                            "hooq_tracing",
                                        ),
                                        ::tracing_core::field::FieldSet::new(
                                            &[
                                                {
                                                    const NAME: ::tracing::__macro_support::FieldName<
                                                        { ::tracing::__macro_support::FieldName::len("path") },
                                                    > = ::tracing::__macro_support::FieldName::new("path");
                                                    NAME.as_str()
                                                },
                                                {
                                                    const NAME: ::tracing::__macro_support::FieldName<
                                                        { ::tracing::__macro_support::FieldName::len("line") },
                                                    > = ::tracing::__macro_support::FieldName::new("line");
                                                    NAME.as_str()
                                                },
                                                {
                                                    const NAME: ::tracing::__macro_support::FieldName<
                                                        { ::tracing::__macro_support::FieldName::len("col") },
                                                    > = ::tracing::__macro_support::FieldName::new("col");
                                                    NAME.as_str()
                                                },
                                                {
                                                    const NAME: ::tracing::__macro_support::FieldName<
                                                        { ::tracing::__macro_support::FieldName::len("error") },
                                                    > = ::tracing::__macro_support::FieldName::new("error");
                                                    NAME.as_str()
                                                },
                                                {
                                                    const NAME: ::tracing::__macro_support::FieldName<
                                                        { ::tracing::__macro_support::FieldName::len("expr") },
                                                    > = ::tracing::__macro_support::FieldName::new("expr");
                                                    NAME.as_str()
                                                },
                                            ],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::INFO
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::INFO
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set_all(
                                            &[
                                                (::tracing::__macro_support::Option::Some(
                                                    &path as &dyn ::tracing::field::Value,
                                                )),
                                                (::tracing::__macro_support::Option::Some(
                                                    &line as &dyn ::tracing::field::Value,
                                                )),
                                                (::tracing::__macro_support::Option::Some(
                                                    &col as &dyn ::tracing::field::Value,
                                                )),
                                                (::tracing::__macro_support::Option::Some(
                                                    &::tracing::field::display(&e)
                                                        as &dyn ::tracing::field::Value,
                                                )),
                                                (::tracing::__macro_support::Option::Some(
                                                    &expr as &dyn ::tracing::field::Value,
                                                )),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                    })
            }
        }
    }
}
pub fn e() -> Result<(), String> {
    {}
    #[allow(clippy::suspicious_else_formatting)]
    {
        let __tracing_attr_span;
        let __tracing_attr_guard;
        if ::tracing::Level::INFO <= ::tracing::level_filters::STATIC_MAX_LEVEL
            && ::tracing::Level::INFO <= ::tracing::level_filters::LevelFilter::current()
            || { false }
        {
            __tracing_attr_span = {
                use ::tracing::__macro_support::Callsite as _;
                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "e",
                            "hooq_tracing",
                            ::tracing::Level::INFO,
                            ::tracing_core::__macro_support::Option::Some(
                                "<hooq_root>/src/hooq_tracing.rs",
                            ),
                            ::tracing_core::__macro_support::Option::Some(35u32),
                            ::tracing_core::__macro_support::Option::Some(
                                "hooq_tracing",
                            ),
                            ::tracing_core::field::FieldSet::new(
                                &[],
                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                            ),
                            ::tracing::metadata::Kind::SPAN,
                        )
                    };
                    ::tracing::callsite::DefaultCallsite::new(&META)
                };
                let mut interest = ::tracing::subscriber::Interest::never();
                if ::tracing::Level::INFO <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::tracing::Level::INFO
                        <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        interest = __CALLSITE.interest();
                        !interest.is_never()
                    }
                    && ::tracing::__macro_support::__is_enabled(
                        __CALLSITE.metadata(),
                        interest,
                    )
                {
                    let meta = __CALLSITE.metadata();
                    ::tracing::Span::new(meta, &{ meta.fields().value_set_all(&[]) })
                } else {
                    let span = ::tracing::__macro_support::__disabled_span(
                        __CALLSITE.metadata(),
                    );
                    {};
                    span
                }
            };
            __tracing_attr_guard = __tracing_attr_span.enter();
        }
        #[warn(clippy::suspicious_else_formatting)]
        {
            #[allow(
                unknown_lints,
                unreachable_code,
                clippy::diverging_sub_expression,
                clippy::empty_loop,
                clippy::let_unit_value,
                clippy::let_with_type_underscore,
                clippy::needless_return,
                clippy::unreachable
            )]
            if false {
                let __tracing_attr_fake_return: Result<(), String> = loop {};
                return __tracing_attr_fake_return;
            }
            {
                d()
                    .inspect_err(|e| {
                        let path = "<hooq_root>/src/hooq_tracing.rs";
                        let line = 37usize;
                        let col = 8usize;
                        let expr = "d()?";
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event <hooq_root>/src/hooq_tracing.rs:34",
                                        "hooq_tracing",
                                        ::tracing::Level::DEBUG,
                                        ::tracing_core::__macro_support::Option::Some(
                                            "<hooq_root>/src/hooq_tracing.rs",
                                        ),
                                        ::tracing_core::__macro_support::Option::Some(34u32),
                                        ::tracing_core::__macro_support::Option::Some(
                                            "hooq_tracing",
                                        ),
                                        ::tracing_core::field::FieldSet::new(
                                            &[
                                                {
                                                    const NAME: ::tracing::__macro_support::FieldName<
                                                        { ::tracing::__macro_support::FieldName::len("path") },
                                                    > = ::tracing::__macro_support::FieldName::new("path");
                                                    NAME.as_str()
                                                },
                                                {
                                                    const NAME: ::tracing::__macro_support::FieldName<
                                                        { ::tracing::__macro_support::FieldName::len("line") },
                                                    > = ::tracing::__macro_support::FieldName::new("line");
                                                    NAME.as_str()
                                                },
                                                {
                                                    const NAME: ::tracing::__macro_support::FieldName<
                                                        { ::tracing::__macro_support::FieldName::len("col") },
                                                    > = ::tracing::__macro_support::FieldName::new("col");
                                                    NAME.as_str()
                                                },
                                                {
                                                    const NAME: ::tracing::__macro_support::FieldName<
                                                        { ::tracing::__macro_support::FieldName::len("error") },
                                                    > = ::tracing::__macro_support::FieldName::new("error");
                                                    NAME.as_str()
                                                },
                                                {
                                                    const NAME: ::tracing::__macro_support::FieldName<
                                                        { ::tracing::__macro_support::FieldName::len("expr") },
                                                    > = ::tracing::__macro_support::FieldName::new("expr");
                                                    NAME.as_str()
                                                },
                                            ],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::DEBUG
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::DEBUG
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set_all(
                                            &[
                                                (::tracing::__macro_support::Option::Some(
                                                    &path as &dyn ::tracing::field::Value,
                                                )),
                                                (::tracing::__macro_support::Option::Some(
                                                    &line as &dyn ::tracing::field::Value,
                                                )),
                                                (::tracing::__macro_support::Option::Some(
                                                    &col as &dyn ::tracing::field::Value,
                                                )),
                                                (::tracing::__macro_support::Option::Some(
                                                    &::tracing::field::display(&e)
                                                        as &dyn ::tracing::field::Value,
                                                )),
                                                (::tracing::__macro_support::Option::Some(
                                                    &expr as &dyn ::tracing::field::Value,
                                                )),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                    })?;
                Err("error!".to_string())
                    .inspect_err(|e| {
                        let path = "<hooq_root>/src/hooq_tracing.rs";
                        let line = 39usize;
                        let col = 5usize;
                        let expr = "Err(\"error!\".to_string())";
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event <hooq_root>/src/hooq_tracing.rs:34",
                                        "hooq_tracing",
                                        ::tracing::Level::DEBUG,
                                        ::tracing_core::__macro_support::Option::Some(
                                            "<hooq_root>/src/hooq_tracing.rs",
                                        ),
                                        ::tracing_core::__macro_support::Option::Some(34u32),
                                        ::tracing_core::__macro_support::Option::Some(
                                            "hooq_tracing",
                                        ),
                                        ::tracing_core::field::FieldSet::new(
                                            &[
                                                {
                                                    const NAME: ::tracing::__macro_support::FieldName<
                                                        { ::tracing::__macro_support::FieldName::len("path") },
                                                    > = ::tracing::__macro_support::FieldName::new("path");
                                                    NAME.as_str()
                                                },
                                                {
                                                    const NAME: ::tracing::__macro_support::FieldName<
                                                        { ::tracing::__macro_support::FieldName::len("line") },
                                                    > = ::tracing::__macro_support::FieldName::new("line");
                                                    NAME.as_str()
                                                },
                                                {
                                                    const NAME: ::tracing::__macro_support::FieldName<
                                                        { ::tracing::__macro_support::FieldName::len("col") },
                                                    > = ::tracing::__macro_support::FieldName::new("col");
                                                    NAME.as_str()
                                                },
                                                {
                                                    const NAME: ::tracing::__macro_support::FieldName<
                                                        { ::tracing::__macro_support::FieldName::len("error") },
                                                    > = ::tracing::__macro_support::FieldName::new("error");
                                                    NAME.as_str()
                                                },
                                                {
                                                    const NAME: ::tracing::__macro_support::FieldName<
                                                        { ::tracing::__macro_support::FieldName::len("expr") },
                                                    > = ::tracing::__macro_support::FieldName::new("expr");
                                                    NAME.as_str()
                                                },
                                            ],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::DEBUG
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::DEBUG
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set_all(
                                            &[
                                                (::tracing::__macro_support::Option::Some(
                                                    &path as &dyn ::tracing::field::Value,
                                                )),
                                                (::tracing::__macro_support::Option::Some(
                                                    &line as &dyn ::tracing::field::Value,
                                                )),
                                                (::tracing::__macro_support::Option::Some(
                                                    &col as &dyn ::tracing::field::Value,
                                                )),
                                                (::tracing::__macro_support::Option::Some(
                                                    &::tracing::field::display(&e)
                                                        as &dyn ::tracing::field::Value,
                                                )),
                                                (::tracing::__macro_support::Option::Some(
                                                    &expr as &dyn ::tracing::field::Value,
                                                )),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                    })
            }
        }
    }
}
pub fn f() -> Result<(), String> {
    {}
    #[allow(clippy::suspicious_else_formatting)]
    {
        let __tracing_attr_span;
        let __tracing_attr_guard;
        if ::tracing::Level::INFO <= ::tracing::level_filters::STATIC_MAX_LEVEL
            && ::tracing::Level::INFO <= ::tracing::level_filters::LevelFilter::current()
            || { false }
        {
            __tracing_attr_span = {
                use ::tracing::__macro_support::Callsite as _;
                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "f",
                            "hooq_tracing",
                            ::tracing::Level::INFO,
                            ::tracing_core::__macro_support::Option::Some(
                                "<hooq_root>/src/hooq_tracing.rs",
                            ),
                            ::tracing_core::__macro_support::Option::Some(43u32),
                            ::tracing_core::__macro_support::Option::Some(
                                "hooq_tracing",
                            ),
                            ::tracing_core::field::FieldSet::new(
                                &[],
                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                            ),
                            ::tracing::metadata::Kind::SPAN,
                        )
                    };
                    ::tracing::callsite::DefaultCallsite::new(&META)
                };
                let mut interest = ::tracing::subscriber::Interest::never();
                if ::tracing::Level::INFO <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::tracing::Level::INFO
                        <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        interest = __CALLSITE.interest();
                        !interest.is_never()
                    }
                    && ::tracing::__macro_support::__is_enabled(
                        __CALLSITE.metadata(),
                        interest,
                    )
                {
                    let meta = __CALLSITE.metadata();
                    ::tracing::Span::new(meta, &{ meta.fields().value_set_all(&[]) })
                } else {
                    let span = ::tracing::__macro_support::__disabled_span(
                        __CALLSITE.metadata(),
                    );
                    {};
                    span
                }
            };
            __tracing_attr_guard = __tracing_attr_span.enter();
        }
        #[warn(clippy::suspicious_else_formatting)]
        {
            #[allow(
                unknown_lints,
                unreachable_code,
                clippy::diverging_sub_expression,
                clippy::empty_loop,
                clippy::let_unit_value,
                clippy::let_with_type_underscore,
                clippy::needless_return,
                clippy::unreachable
            )]
            if false {
                let __tracing_attr_fake_return: Result<(), String> = loop {};
                return __tracing_attr_fake_return;
            }
            {
                e()
                    .inspect_err(|e| {
                        let path = "<hooq_root>/src/hooq_tracing.rs";
                        let line = 45usize;
                        let col = 8usize;
                        let expr = "e()?";
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event <hooq_root>/src/hooq_tracing.rs:42",
                                        "hooq_tracing",
                                        ::tracing::Level::TRACE,
                                        ::tracing_core::__macro_support::Option::Some(
                                            "<hooq_root>/src/hooq_tracing.rs",
                                        ),
                                        ::tracing_core::__macro_support::Option::Some(42u32),
                                        ::tracing_core::__macro_support::Option::Some(
                                            "hooq_tracing",
                                        ),
                                        ::tracing_core::field::FieldSet::new(
                                            &[
                                                {
                                                    const NAME: ::tracing::__macro_support::FieldName<
                                                        { ::tracing::__macro_support::FieldName::len("path") },
                                                    > = ::tracing::__macro_support::FieldName::new("path");
                                                    NAME.as_str()
                                                },
                                                {
                                                    const NAME: ::tracing::__macro_support::FieldName<
                                                        { ::tracing::__macro_support::FieldName::len("line") },
                                                    > = ::tracing::__macro_support::FieldName::new("line");
                                                    NAME.as_str()
                                                },
                                                {
                                                    const NAME: ::tracing::__macro_support::FieldName<
                                                        { ::tracing::__macro_support::FieldName::len("col") },
                                                    > = ::tracing::__macro_support::FieldName::new("col");
                                                    NAME.as_str()
                                                },
                                                {
                                                    const NAME: ::tracing::__macro_support::FieldName<
                                                        { ::tracing::__macro_support::FieldName::len("error") },
                                                    > = ::tracing::__macro_support::FieldName::new("error");
                                                    NAME.as_str()
                                                },
                                                {
                                                    const NAME: ::tracing::__macro_support::FieldName<
                                                        { ::tracing::__macro_support::FieldName::len("expr") },
                                                    > = ::tracing::__macro_support::FieldName::new("expr");
                                                    NAME.as_str()
                                                },
                                            ],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::TRACE
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::TRACE
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set_all(
                                            &[
                                                (::tracing::__macro_support::Option::Some(
                                                    &path as &dyn ::tracing::field::Value,
                                                )),
                                                (::tracing::__macro_support::Option::Some(
                                                    &line as &dyn ::tracing::field::Value,
                                                )),
                                                (::tracing::__macro_support::Option::Some(
                                                    &col as &dyn ::tracing::field::Value,
                                                )),
                                                (::tracing::__macro_support::Option::Some(
                                                    &::tracing::field::display(&e)
                                                        as &dyn ::tracing::field::Value,
                                                )),
                                                (::tracing::__macro_support::Option::Some(
                                                    &expr as &dyn ::tracing::field::Value,
                                                )),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                    })?;
                Err("error!".to_string())
                    .inspect_err(|e| {
                        let path = "<hooq_root>/src/hooq_tracing.rs";
                        let line = 47usize;
                        let col = 5usize;
                        let expr = "Err(\"error!\".to_string())";
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event <hooq_root>/src/hooq_tracing.rs:42",
                                        "hooq_tracing",
                                        ::tracing::Level::TRACE,
                                        ::tracing_core::__macro_support::Option::Some(
                                            "<hooq_root>/src/hooq_tracing.rs",
                                        ),
                                        ::tracing_core::__macro_support::Option::Some(42u32),
                                        ::tracing_core::__macro_support::Option::Some(
                                            "hooq_tracing",
                                        ),
                                        ::tracing_core::field::FieldSet::new(
                                            &[
                                                {
                                                    const NAME: ::tracing::__macro_support::FieldName<
                                                        { ::tracing::__macro_support::FieldName::len("path") },
                                                    > = ::tracing::__macro_support::FieldName::new("path");
                                                    NAME.as_str()
                                                },
                                                {
                                                    const NAME: ::tracing::__macro_support::FieldName<
                                                        { ::tracing::__macro_support::FieldName::len("line") },
                                                    > = ::tracing::__macro_support::FieldName::new("line");
                                                    NAME.as_str()
                                                },
                                                {
                                                    const NAME: ::tracing::__macro_support::FieldName<
                                                        { ::tracing::__macro_support::FieldName::len("col") },
                                                    > = ::tracing::__macro_support::FieldName::new("col");
                                                    NAME.as_str()
                                                },
                                                {
                                                    const NAME: ::tracing::__macro_support::FieldName<
                                                        { ::tracing::__macro_support::FieldName::len("error") },
                                                    > = ::tracing::__macro_support::FieldName::new("error");
                                                    NAME.as_str()
                                                },
                                                {
                                                    const NAME: ::tracing::__macro_support::FieldName<
                                                        { ::tracing::__macro_support::FieldName::len("expr") },
                                                    > = ::tracing::__macro_support::FieldName::new("expr");
                                                    NAME.as_str()
                                                },
                                            ],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::TRACE
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::TRACE
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set_all(
                                            &[
                                                (::tracing::__macro_support::Option::Some(
                                                    &path as &dyn ::tracing::field::Value,
                                                )),
                                                (::tracing::__macro_support::Option::Some(
                                                    &line as &dyn ::tracing::field::Value,
                                                )),
                                                (::tracing::__macro_support::Option::Some(
                                                    &col as &dyn ::tracing::field::Value,
                                                )),
                                                (::tracing::__macro_support::Option::Some(
                                                    &::tracing::field::display(&e)
                                                        as &dyn ::tracing::field::Value,
                                                )),
                                                (::tracing::__macro_support::Option::Some(
                                                    &expr as &dyn ::tracing::field::Value,
                                                )),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                    })
            }
        }
    }
}
