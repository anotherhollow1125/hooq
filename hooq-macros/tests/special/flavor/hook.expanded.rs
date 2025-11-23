mod trait_define {
    pub trait CustomHook {
        fn hook(self, f: impl FnOnce() -> hooq::HooqMeta) -> Self;
    }
    impl<T, E> CustomHook for Result<T, E> {
        fn hook(self, f: impl FnOnce() -> hooq::HooqMeta) -> Self {
            let info = f();
            {
                ::std::io::_eprint(format_args!("{0:?}\n", info));
            };
            if let Some(val) = info.get_binding::<&str>("hoge") {
                {
                    ::std::io::_eprint(format_args!("hoge: {0}\n", val));
                };
            }
            self
        }
    }
}
mod custom {
    use hooq::hooq;
    #[allow(unused)]
    use super::trait_define::CustomHook as _;
    pub fn use_hook<T, E>(result: Result<T, E>) -> Result<T, E>
    where
        E: std::fmt::Debug,
    {
        result
            .hook(|| {
                ::hooq::HooqMeta {
                    line: 29usize,
                    column: 9usize,
                    path: "<hooq_root>/tests/special/flavor/hook.rs",
                    file: "hook.rs",
                    source_str: "result",
                    count: "1st tail expr",
                    bindings: ::std::collections::HashMap::from([
                        (
                            ::std::string::ToString::to_string("hoge"),
                            {
                                let expr = ::std::string::ToString::to_string("\"val\"");
                                let value: ::std::rc::Rc<dyn ::std::any::Any> = ::std::rc::Rc::new(
                                    "val",
                                );
                                ::hooq::BindingPayload {
                                    expr,
                                    value,
                                }
                            },
                        ),
                    ]),
                }
            })
    }
    pub fn use_hook2<T, E>(result: Result<T, E>) -> Result<T, E>
    where
        E: std::fmt::Debug,
    {
        result
            .hook(|| {
                ::hooq::HooqMeta {
                    line: 37usize,
                    column: 9usize,
                    path: "<hooq_root>/tests/special/flavor/hook.rs",
                    file: "hook.rs",
                    source_str: "result",
                    count: "1st tail expr",
                    bindings: ::std::collections::HashMap::from([]),
                }
            })
    }
}
mod custom2 {
    use hooq_macros::hooq;
    use super::trait_define::CustomHook;
    pub fn use_hook<T, E>(result: Result<T, E>) -> Result<T, E>
    where
        E: std::fmt::Debug,
    {
        result
            .hook(|| {
                ::hooq::HooqMeta {
                    line: 51usize,
                    column: 9usize,
                    path: "<hooq_root>/tests/special/flavor/hook.rs",
                    file: "hook.rs",
                    source_str: "result",
                    count: "1st tail expr",
                    bindings: ::std::collections::HashMap::from([]),
                }
            })
    }
    pub fn use_hook2<T, E>(result: Result<T, E>) -> Result<T, E>
    where
        E: std::fmt::Debug,
    {
        result
            .hook(|| {
                ::hooq::HooqMeta {
                    line: 59usize,
                    column: 9usize,
                    path: "<hooq_root>/tests/special/flavor/hook.rs",
                    file: "hook.rs",
                    source_str: "result",
                    count: "1st tail expr",
                    bindings: ::std::collections::HashMap::from([]),
                }
            })
    }
}
