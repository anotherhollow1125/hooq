use hooq_macros::hooq;
mod trait_define {
    use std::fmt::Debug;
    pub trait CustomHook {
        fn hook(self, f: impl FnOnce() -> hooq::HooqMeta) -> Self;
    }
    impl<T, E> CustomHook for Result<T, E>
    where
        T: Debug,
        E: Debug,
    {
        fn hook(self, f: impl FnOnce() -> hooq::HooqMeta) -> Self {
            let info = f();
            {
                ::std::io::_eprint(format_args!("{0:?}\n", info));
            };
            if let Some(detail) = info.get_binding::<bool>("detail") && *detail {
                {
                    ::std::io::_print(format_args!("detail: {0:?}\n", self));
                };
            }
            self
        }
    }
}
mod custom {
    use hooq_macros::hooq;
    #[allow(unused)]
    use super::trait_define::CustomHook as _;
    pub fn use_hook(result: Result<i32, ()>) -> Result<i32, ()> {
        if let Ok(val) = result && val >= 100 {
            return Ok(100)
                .hook(|| {
                    ::hooq::HooqMeta {
                        line: 42usize,
                        column: 13usize,
                        path: "<hooq_root>/tests/special/meta_var/hooq_meta.rs",
                        abs_path: "<hooq_root>/tests/special/meta_var/hooq_meta.rs",
                        file: "hooq_meta.rs",
                        expr: "Ok(100)",
                        count: "1st return",
                        bindings: ::std::collections::HashMap::from([
                            (
                                ::std::string::ToString::to_string("array"),
                                {
                                    let expr = ::std::string::ToString::to_string("[1, 2, 3]");
                                    let value: ::std::rc::Rc<dyn ::std::any::Any> = ::std::rc::Rc::new([
                                        1, 2, 3,
                                    ]);
                                    ::hooq::BindingPayload {
                                        expr,
                                        value,
                                    }
                                },
                            ),
                            (
                                ::std::string::ToString::to_string("detail"),
                                {
                                    let expr = ::std::string::ToString::to_string("true");
                                    let value: ::std::rc::Rc<dyn ::std::any::Any> = ::std::rc::Rc::new(
                                        true,
                                    );
                                    ::hooq::BindingPayload {
                                        expr,
                                        value,
                                    }
                                },
                            ),
                            (
                                ::std::string::ToString::to_string("hoge"),
                                {
                                    let expr = ::std::string::ToString::to_string(
                                        "\"hogehoge\"",
                                    );
                                    let value: ::std::rc::Rc<dyn ::std::any::Any> = ::std::rc::Rc::new(
                                        "hogehoge",
                                    );
                                    ::hooq::BindingPayload {
                                        expr,
                                        value,
                                    }
                                },
                            ),
                            (
                                ::std::string::ToString::to_string("inner_struct"),
                                {
                                    let expr = ::std::string::ToString::to_string(
                                        "InnerStruct",
                                    );
                                    let value: ::std::rc::Rc<dyn ::std::any::Any> = ::std::rc::Rc::new(
                                        InnerStruct,
                                    );
                                    ::hooq::BindingPayload {
                                        expr,
                                        value,
                                    }
                                },
                            ),
                        ]),
                    }
                });
        }
        struct InnerStruct;
        #[automatically_derived]
        impl ::core::fmt::Debug for InnerStruct {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "InnerStruct")
            }
        }
        result
            .hook(|| {
                ::hooq::HooqMeta {
                    line: 51usize,
                    column: 9usize,
                    path: "<hooq_root>/tests/special/meta_var/hooq_meta.rs",
                    abs_path: "<hooq_root>/tests/special/meta_var/hooq_meta.rs",
                    file: "hooq_meta.rs",
                    expr: "result",
                    count: "1st tail expr",
                    bindings: ::std::collections::HashMap::from([
                        (
                            ::std::string::ToString::to_string("array"),
                            {
                                let expr = ::std::string::ToString::to_string("[1, 2, 3]");
                                let value: ::std::rc::Rc<dyn ::std::any::Any> = ::std::rc::Rc::new([
                                    1, 2, 3,
                                ]);
                                ::hooq::BindingPayload {
                                    expr,
                                    value,
                                }
                            },
                        ),
                        (
                            ::std::string::ToString::to_string("bool_val"),
                            {
                                let expr = ::std::string::ToString::to_string("true");
                                let value: ::std::rc::Rc<dyn ::std::any::Any> = ::std::rc::Rc::new(
                                    true,
                                );
                                ::hooq::BindingPayload {
                                    expr,
                                    value,
                                }
                            },
                        ),
                        (
                            ::std::string::ToString::to_string("fuga"),
                            {
                                let expr = ::std::string::ToString::to_string("10");
                                let value: ::std::rc::Rc<dyn ::std::any::Any> = ::std::rc::Rc::new(
                                    10,
                                );
                                ::hooq::BindingPayload {
                                    expr,
                                    value,
                                }
                            },
                        ),
                        (
                            ::std::string::ToString::to_string("hoge"),
                            {
                                let expr = ::std::string::ToString::to_string(
                                    "\"hogehoge\"",
                                );
                                let value: ::std::rc::Rc<dyn ::std::any::Any> = ::std::rc::Rc::new(
                                    "hogehoge",
                                );
                                ::hooq::BindingPayload {
                                    expr,
                                    value,
                                }
                            },
                        ),
                        (
                            ::std::string::ToString::to_string("inner_struct"),
                            {
                                let expr = ::std::string::ToString::to_string(
                                    "InnerStruct",
                                );
                                let value: ::std::rc::Rc<dyn ::std::any::Any> = ::std::rc::Rc::new(
                                    InnerStruct,
                                );
                                ::hooq::BindingPayload {
                                    expr,
                                    value,
                                }
                            },
                        ),
                        (
                            ::std::string::ToString::to_string("tuple"),
                            {
                                let expr = ::std::string::ToString::to_string(
                                    "(InnerStruct, 10)",
                                );
                                let value: ::std::rc::Rc<dyn ::std::any::Any> = ::std::rc::Rc::new((
                                    InnerStruct,
                                    10,
                                ));
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
}
fn using_hooq_meta(flag: bool) -> Result<(), ()> {
    if flag {
        Ok(())
            .inspect(|_| {
                let meta = ::hooq::HooqMeta {
                    line: 67usize,
                    column: 9usize,
                    path: "<hooq_root>/tests/special/meta_var/hooq_meta.rs",
                    abs_path: "<hooq_root>/tests/special/meta_var/hooq_meta.rs",
                    file: "hooq_meta.rs",
                    expr: "Ok(())",
                    count: "1st tail expr",
                    bindings: ::std::collections::HashMap::from([
                        (
                            ::std::string::ToString::to_string("hoge"),
                            {
                                let expr = ::std::string::ToString::to_string("\"hoge\"");
                                let value: ::std::rc::Rc<dyn ::std::any::Any> = ::std::rc::Rc::new(
                                    "hoge",
                                );
                                ::hooq::BindingPayload {
                                    expr,
                                    value,
                                }
                            },
                        ),
                    ]),
                };
                {
                    ::std::io::_print(format_args!("{0:?}\n", meta));
                };
                if let Some(fuga) = meta.get_binding::<&'static str>("fuga") {
                    {
                        ::std::io::_print(format_args!("fuga: {0}\n", fuga));
                    };
                }
            })
    } else {
        Ok(())
            .inspect(|_| {
                let meta = ::hooq::HooqMeta {
                    line: 70usize,
                    column: 9usize,
                    path: "<hooq_root>/tests/special/meta_var/hooq_meta.rs",
                    abs_path: "<hooq_root>/tests/special/meta_var/hooq_meta.rs",
                    file: "hooq_meta.rs",
                    expr: "Ok(())",
                    count: "2nd tail expr",
                    bindings: ::std::collections::HashMap::from([
                        (
                            ::std::string::ToString::to_string("fuga"),
                            {
                                let expr = ::std::string::ToString::to_string(
                                    "\"fugafuga\"",
                                );
                                let value: ::std::rc::Rc<dyn ::std::any::Any> = ::std::rc::Rc::new(
                                    "fugafuga",
                                );
                                ::hooq::BindingPayload {
                                    expr,
                                    value,
                                }
                            },
                        ),
                        (
                            ::std::string::ToString::to_string("hoge"),
                            {
                                let expr = ::std::string::ToString::to_string("\"hoge\"");
                                let value: ::std::rc::Rc<dyn ::std::any::Any> = ::std::rc::Rc::new(
                                    "hoge",
                                );
                                ::hooq::BindingPayload {
                                    expr,
                                    value,
                                }
                            },
                        ),
                    ]),
                };
                {
                    ::std::io::_print(format_args!("{0:?}\n", meta));
                };
                if let Some(fuga) = meta.get_binding::<&'static str>("fuga") {
                    {
                        ::std::io::_print(format_args!("fuga: {0}\n", fuga));
                    };
                }
            })
    }
        .inspect(|_| {
            let meta = ::hooq::HooqMeta {
                line: 66usize,
                column: 5usize,
                path: "<hooq_root>/tests/special/meta_var/hooq_meta.rs",
                abs_path: "<hooq_root>/tests/special/meta_var/hooq_meta.rs",
                file: "hooq_meta.rs",
                expr: "if flag { Ok(()) } else { #[hooq :: fuga = \"fugafuga\"] Ok(()) }",
                count: "3rd tail expr",
                bindings: ::std::collections::HashMap::from([
                    (
                        ::std::string::ToString::to_string("hoge"),
                        {
                            let expr = ::std::string::ToString::to_string("\"hoge\"");
                            let value: ::std::rc::Rc<dyn ::std::any::Any> = ::std::rc::Rc::new(
                                "hoge",
                            );
                            ::hooq::BindingPayload {
                                expr,
                                value,
                            }
                        },
                    ),
                ]),
            };
            {
                ::std::io::_print(format_args!("{0:?}\n", meta));
            };
            if let Some(fuga) = meta.get_binding::<&'static str>("fuga") {
                {
                    ::std::io::_print(format_args!("fuga: {0}\n", fuga));
                };
            }
        })
}
