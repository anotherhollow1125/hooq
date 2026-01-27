#![allow(unused_braces)]
#![allow(clippy::let_unit_value)]
use std::sync::LazyLock;
use hooq_macros::hooq;
use util_macros::id;
fn enresult<T>(t: T) -> Result<T, ()> {
    Ok(t)
}
fn skip_stmts() -> Result<(), ()> {
    let _ = enresult(true)?
        && {
            enresult(true)
                .inspect(|_| {
                    {
                        ::std::io::_print(
                            format_args!("tag: {0}\n", "sub scope in local"),
                        );
                    };
                })?
        };
    fn _fnc() -> Result<(), ()> {
        enresult(())
            .inspect(|_| {
                {
                    ::std::io::_print(format_args!("tag: {0}\n", "sub scope in item"));
                };
            })?;
        if true {
            return Err(())
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0}\n", "item"));
                    };
                });
        }
        Err(())
            .inspect(|_| {
                {
                    ::std::io::_print(format_args!("tag: {0}\n", "item"));
                };
            })
    }
    {
        {
            ::std::io::_print(format_args!("sub scope in expr\n"));
        };
        if enresult(false)
            .inspect(|_| {
                {
                    ::std::io::_print(format_args!("tag: {0}\n", "expr"));
                };
            })?
        {
            return Ok({
                enresult(enresult(()))
                    .inspect(|_| {
                        {
                            ::std::io::_print(format_args!("tag: {0}\n", "expr"));
                        };
                    })?
            })?;
        }
        enresult(())
    }?;
    {
        ::std::io::_print(
            format_args!(
                "{0}, {1}\n", enresult(10) ?, { enresult(()).inspect(| _ | { {
                ::std::io::_print(format_args!("tag: {0}\n", "macro")); }; }) ?;
                Result::< u32, () >::Ok(20).inspect(| _ | { {
                ::std::io::_print(format_args!("tag: {0}\n", "macro")); }; }) } ?
            ),
        );
    };
    {
        ::std::io::_print(
            format_args!(
                "{0}, {1}\n", enresult(10).inspect(| _ | { {
                ::std::io::_print(format_args!("tag: {0}\n", "macro")); }; }) ?, {
                enresult(()).inspect(| _ | { {
                ::std::io::_print(format_args!("tag: {0}\n", "macro")); }; }) ?;
                Result::< u32, () >::Ok(20).inspect(| _ | { {
                ::std::io::_print(format_args!("tag: {0}\n", "macro")); }; }) } ?
            ),
        );
    };
    Err(())
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
            };
        })
}
fn skip_item() -> Result<(), ()> {
    struct _S;
    impl _S {
        fn _method() -> Result<(), ()> {
            Err(())
                .inspect(|_| {
                    {
                        ::std::io::_print(
                            format_args!("tag: {0}\n", "sub scope in impl"),
                        );
                    };
                })
        }
        fn _method2() -> Result<(), ()> {
            Err(())
                .inspect(|_| {
                    {
                        ::std::io::_print(
                            format_args!("tag: {0}\n", "sub scope in impl"),
                        );
                    };
                })
        }
        fn _method3(&self) -> Result<(), ()> {
            Err(())
                .inspect(|_| {
                    {
                        ::std::io::_print(
                            format_args!("tag: {0}\n", "sub scope in impl"),
                        );
                    };
                })
        }
        fn _method4(&self) -> Result<(), ()> {
            Err(())
                .inspect(|_| {
                    {
                        ::std::io::_print(
                            format_args!("tag: {0}\n", "sub scope in impl"),
                        );
                    };
                })
        }
    }
    mod m {
        fn _mod_fn() -> Result<(), ()> {
            Err(())
                .inspect(|_| {
                    {
                        ::std::io::_print(
                            format_args!("tag: {0}\n", "sub scope in mod"),
                        );
                    };
                })
        }
        fn _mod_fn2() -> Result<(), ()> {
            Err(())
                .inspect(|_| {
                    {
                        ::std::io::_print(
                            format_args!("tag: {0}\n", "sub scope in mod"),
                        );
                    };
                })
        }
    }
    const _C: usize = {
        let _ = || -> Result<u32, ()> {
            let res = enresult(42_u32)
                .inspect(|_| {
                    {
                        ::std::io::_print(
                            format_args!("tag: {0}\n", "sub scope in const"),
                        );
                    };
                })?;
            Ok(res)
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0}\n", "const"));
                    };
                })
        };
        10
    };
    static _SS: LazyLock<u32> = LazyLock::new(|| {
        (|| -> Result<u32, ()> {
            let res = enresult(42_u32)
                .inspect(|_| {
                    {
                        ::std::io::_print(
                            format_args!("tag: {0}\n", "sub scope in static"),
                        );
                    };
                })?;
            Ok(res)
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0}\n", "static"));
                    };
                })
        })()
            .unwrap_or(0)
    });
    trait _T {
        fn _trait_method() -> Result<(), ()> {
            Err(())
                .inspect(|_| {
                    {
                        ::std::io::_print(
                            format_args!("tag: {0}\n", "sub scope in trait"),
                        );
                    };
                })
        }
        fn _trait_method2() -> Result<(), ()> {
            Err(())
                .inspect(|_| {
                    {
                        ::std::io::_print(
                            format_args!("tag: {0}\n", "sub scope in trait"),
                        );
                    };
                })
        }
        fn _trait_method3(&self) -> Result<(), ()> {
            Err(())
                .inspect(|_| {
                    {
                        ::std::io::_print(
                            format_args!("tag: {0}\n", "sub scope in impl"),
                        );
                    };
                })
        }
        fn _trait_method4(&self) -> Result<(), ()> {
            Err(())
                .inspect(|_| {
                    {
                        ::std::io::_print(
                            format_args!("tag: {0}\n", "sub scope in impl"),
                        );
                    };
                })
        }
    }
    mod tmp {
        use super::*;
        fn _macro_fn() -> Result<(), ()> {
            Err(())
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0}\n", "macro"));
                    };
                })
        }
        #[allow(clippy::needless_question_mark)]
        fn _macro_fn_2() -> Result<(), ()> {
            Ok(
                {
                    enresult(())
                        .inspect(|_| {
                            {
                                ::std::io::_print(format_args!("tag: {0}\n", "macro"));
                            };
                        })?;
                    Result::<(), ()>::Err(())
                        .inspect(|_| {
                            {
                                ::std::io::_print(format_args!("tag: {0}\n", "macro"));
                            };
                        })
                }?,
            )
        }
    }
    Err(())
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
            };
        })
}
fn skip_expr() -> Result<(), ()> {
    let _ = { enresult(())? };
    let _ = {
        enresult((
            enresult(())?,
            {
                enresult(())
                    .inspect(|_| {
                        {
                            ::std::io::_print(
                                format_args!("tag: {0}\n", "sub scope in function args."),
                            );
                        };
                    })?
            },
        ))?
    };
    let _f = |f: bool| -> Result<((), ()), ()> {
        if f {
            return Ok((
                enresult(())?,
                {
                    enresult(())
                        .inspect(|_| {
                            {
                                ::std::io::_print(format_args!("tag: {0}\n", "return"));
                            };
                        })?
                },
            ));
        }
        Ok(((), ()))
            .inspect(|_| {
                {
                    ::std::io::_print(format_args!("tag: {0}\n", "return"));
                };
            })
    };
    let _ = [
        enresult(1)?,
        enresult(2)?,
        {
            enresult(3)
                .inspect(|_| {
                    {
                        ::std::io::_print(
                            format_args!("tag: {0}\n", "sub scope in array"),
                        );
                    };
                })?
        },
    ];
    let _ = {
        let _ = enresult(
                enresult(())
                    .inspect(|_| {
                        {
                            ::std::io::_print(format_args!("tag: {0}\n", "block"));
                        };
                    })?,
            )
            .inspect(|_| {
                {
                    ::std::io::_print(format_args!("tag: {0}\n", "block"));
                };
            })?;
        enresult(
                enresult(enresult(()))
                    .inspect(|_| {
                        {
                            ::std::io::_print(format_args!("tag: {0}\n", "block"));
                        };
                    })?,
            )
            .inspect(|_| {
                {
                    ::std::io::_print(format_args!("tag: {0}\n", "block"));
                };
            })?
    }?;
    enresult(
        enresult(
            enresult({
                enresult(())
                    .inspect(|_| {
                        {
                            ::std::io::_print(
                                format_args!("tag: {0}\n", "sub scope in call"),
                            );
                        };
                    })?
            })?,
        )?,
    )?;
    ((
        enresult(|| {
            enresult(())
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0}\n", "call-2"));
                    };
                })?;
            enresult(())
        })?,
        {
            enresult(())
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0}\n", "call-2"));
                    };
                })?;
        },
    )
        .0)()?;
    let _ = (
        enresult(10)?,
        {
            enresult(())
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0}\n", "cast"));
                    };
                })?
        },
    )
        .0 as u64;
    let _ = |f: bool| -> Result<(), ()> {
        enresult(())
            .inspect(|_| {
                {
                    ::std::io::_print(
                        format_args!("tag: {0}\n", "sub scope in closure"),
                    );
                };
            })?;
        let _ = enresult(
                enresult(())
                    .inspect(|_| {
                        {
                            ::std::io::_print(format_args!("tag: {0}\n", "closure"));
                        };
                    })?,
            )
            .inspect(|_| {
                {
                    ::std::io::_print(format_args!("tag: {0}\n", "closure"));
                };
            })?;
        if f {
            return Err(())
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0}\n", "closure"));
                    };
                });
        }
        enresult(
                enresult(enresult(()))
                    .inspect(|_| {
                        {
                            ::std::io::_print(format_args!("tag: {0}\n", "closure"));
                        };
                    })?,
            )
            .inspect(|_| {
                {
                    ::std::io::_print(format_args!("tag: {0}\n", "closure"));
                };
            })?
            .inspect(|_| {
                {
                    ::std::io::_print(format_args!("tag: {0}\n", "closure"));
                };
            })
    };
    const _C: () = const {
        let _ = || {
            enresult(())
                .inspect(|_| {
                    {
                        ::std::io::_print(
                            format_args!("tag: {0}\n", "sub scope in const block"),
                        );
                    };
                })?;
            Result::<(), ()>::Err(())
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0}\n", "const block"));
                    };
                })
        };
    };
    struct Strct {
        #[allow(unused)]
        field: (),
        #[allow(unused)]
        field2: (),
    }
    let _ = Strct {
        field: ((
            enresult(())?,
            {
                enresult(())
                    .inspect(|_| {
                        {
                            ::std::io::_print(
                                format_args!("tag: {0}\n", "sub scope in struct field"),
                            );
                        };
                    })?
            },
        )
            .0),
        field2: ((
            enresult(())?,
            {
                enresult(())
                    .inspect(|_| {
                        {
                            ::std::io::_print(
                                format_args!("tag: {0}\n", "sub scope in struct field"),
                            );
                        };
                    })?
            },
        )
            .0),
    };
    let _ = Strct {
        field: ((
            enresult(())?,
            {
                enresult(())
                    .inspect(|_| {
                        {
                            ::std::io::_print(
                                format_args!("tag: {0}\n", "sub scope in struct field"),
                            );
                        };
                    })?
            },
        )
            .0),
        field2: ((
            enresult(())
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0}\n", "field"));
                    };
                })?,
            {
                enresult(())
                    .inspect(|_| {
                        {
                            ::std::io::_print(
                                format_args!("tag: {0}\n", "sub scope in struct field"),
                            );
                        };
                    })?
            },
        )
            .0),
    };
    for _ in enresult([
        {
            enresult(())
                .inspect(|_| {
                    {
                        ::std::io::_print(
                            format_args!("tag: {0}\n", "sub scope in for loop expr"),
                        );
                    };
                })?
        },
    ])? {
        enresult(())
            .inspect(|_| {
                {
                    ::std::io::_print(
                        format_args!("tag: {0}\n", "sub scope in for loop"),
                    );
                };
            })?;
    }
    if enresult({
        enresult(true)
            .inspect(|_| {
                {
                    ::std::io::_print(
                        format_args!("tag: {0}\n", "sub scope in if condition"),
                    );
                };
            })?
    })? {
        enresult(())
            .inspect(|_| {
                {
                    ::std::io::_print(format_args!("tag: {0}\n", "sub scope in if"));
                };
            })?;
    } else if enresult({
            enresult(true)
                .inspect(|_| {
                    {
                        ::std::io::_print(
                            format_args!("tag: {0}\n", "sub scope in if condition"),
                        );
                    };
                })?
        })
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0}\n", "if"));
            };
        })?
    {
        enresult(())
            .inspect(|_| {
                {
                    ::std::io::_print(
                        format_args!("tag: {0}\n", "sub scope in else if"),
                    );
                };
            })?;
    } else {
        enresult(())
            .inspect(|_| {
                {
                    ::std::io::_print(format_args!("tag: {0}\n", "sub scope in else"));
                };
            })?;
    };
    let _ = enresult([
        enresult(())?,
        {
            enresult(())
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0}\n", "index"));
                    };
                })?
        },
    ])?[(
        enresult(0)?,
        {
            enresult(())
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0}\n", "index"));
                    };
                })?
        },
    )
        .0];
    if let Err(()) = enresult(())
        && let Err(()) = enresult(
            (
                enresult(enresult(()))?,
                {
                    enresult(())
                        .inspect(|_| {
                            {
                                ::std::io::_print(format_args!("tag: {0}\n", "let"));
                            };
                        })?
                },
            )
                .0,
        )?
    {
        enresult(())
            .inspect(|_| {
                {
                    ::std::io::_print(format_args!("tag: {0}\n", "let"));
                };
            })?;
    }
    let mut i = 0;
    let _ = loop {
        if i > 0 {
            break enresult(());
        }
        i += 1;
        enresult(())
            .inspect(|_| {
                {
                    ::std::io::_print(format_args!("tag: {0}\n", "loop"));
                };
            })?;
    }?;
    let _ = <[_]>::into_vec(
        ::alloc::boxed::box_new([
            enresult(0)?,
            enresult(1)?,
            {
                enresult(2)
                    .inspect(|_| {
                        {
                            ::std::io::_print(
                                format_args!("tag: {0}\n", "sub scope in macro-outer"),
                            );
                        };
                    })?
            },
        ]),
    );
    let _ = <[_]>::into_vec(
        ::alloc::boxed::box_new([
            <[_]>::into_vec(
                ::alloc::boxed::box_new([
                    enresult(0)?,
                    enresult(1)?,
                    {
                        enresult(2)
                            .inspect(|_| {
                                {
                                    ::std::io::_print(
                                        format_args!("tag: {0}\n", "sub scope in macro-inner"),
                                    );
                                };
                            })?
                    },
                ]),
            ),
        ]),
    );
    #[allow(clippy::unit_arg)]
    let _ = match enresult(
        Some({
            enresult(())
                .inspect(|_| {
                    {
                        ::std::io::_print(
                            format_args!("tag: {0}\n", "sub scope in match expr"),
                        );
                    };
                })?
        }),
    )? {
        Some(
            (),
        ) if enresult(true)
            .inspect(|_| {
                {
                    ::std::io::_print(format_args!("tag: {0}\n", "match"));
                };
            })? => {
            enresult(())
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0}\n", "match"));
                    };
                })?;
            enresult(())
        }
        Some(()) => {
            enresult(())
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0}\n", "match"));
                    };
                })?;
            enresult(())
        }
        None if enresult(true)
            .inspect(|_| {
                {
                    ::std::io::_print(format_args!("tag: {0}\n", "match"));
                };
            })? => {
            Err(())
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0}\n", "match"));
                    };
                })
        }
        None => enresult(()),
    }?;
    impl Strct {
        fn method(&self, _: ()) -> Result<&Self, ()> {
            Ok(self)
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
                    };
                })
        }
    }
    let s = Strct { field: (), field2: () };
    s.method(enresult(())?)?
        .method({
            enresult(())
                .inspect(|_| {
                    {
                        ::std::io::_print(
                            format_args!("tag: {0}\n", "sub scope in method call"),
                        );
                    };
                })?;
            enresult(())
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0}\n", "method_call"));
                    };
                })?
        })?;
    let _ = (enresult({
        enresult(enresult(()))
            .inspect(|_| {
                {
                    ::std::io::_print(format_args!("tag: {0}\n", "paren"));
                };
            })?
    })?)?;
    let _ = enresult(
        (
            enresult(0)?,
            {
                enresult(())
                    .inspect(|_| {
                        {
                            ::std::io::_print(format_args!("tag: {0}\n", "range-1"));
                        };
                    })?;
            },
        )
            .0,
    )?..enresult(
            (
                enresult(1)
                    .inspect(|_| {
                        {
                            ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
                        };
                    })?,
                {
                    enresult(())
                        .inspect(|_| {
                            {
                                ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
                            };
                        })?;
                },
            )
                .0,
        )
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
            };
        })?;
    let _ = enresult(
            (
                enresult(0)
                    .inspect(|_| {
                        {
                            ::std::io::_print(format_args!("tag: {0}\n", "range-2"));
                        };
                    })?,
                {
                    enresult(())
                        .inspect(|_| {
                            {
                                ::std::io::_print(format_args!("tag: {0}\n", "range-2"));
                            };
                        })?;
                },
            )
                .0,
        )
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0}\n", "range-2"));
            };
        })?..(enresult(
        (
            enresult(1)?,
            {
                enresult(())
                    .inspect(|_| {
                        {
                            ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
                        };
                    })?;
            },
        )
            .0,
    )?);
    let _ = [enresult(
        (
            enresult(0)?,
            {
                enresult(())
                    .inspect(|_| {
                        {
                            ::std::io::_print(format_args!("tag: {0}\n", "repeat"));
                        };
                    })?
            },
        )
            .0,
    )?; {
        fn _f() -> Result<(), ()> {
            enresult(())
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0}\n", "repeat"));
                    };
                })?;
            Err(())
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0}\n", "repeat"));
                    };
                })
        }
        5
    }];
    let _ = (
        enresult(())?,
        {
            enresult(())
                .inspect(|_| {
                    {
                        ::std::io::_print(
                            format_args!("tag: {0}\n", "sub scope in tuple"),
                        );
                    };
                })?
        },
    );
    let _ = !enresult(
        (
            enresult(true)?,
            {
                enresult(())
                    .inspect(|_| {
                        {
                            ::std::io::_print(format_args!("tag: {0}\n", "unary"));
                        };
                    })?
            },
        )
            .0,
    )?;
    let _ = unsafe {
        unsafe fn unsafe_enresult<T>(value: T) -> Result<T, ()> {
            Ok(value)
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0}\n", "unsafe"));
                    };
                })
        }
        unsafe_enresult(())
            .inspect(|_| {
                {
                    ::std::io::_print(format_args!("tag: {0}\n", "sub scope in unsafe"));
                };
            })?;
        unsafe_enresult(())
    }?;
    while (
        enresult(false)?,
        {
            enresult(())
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0}\n", "while"));
                    };
                })?;
        },
    )
        .0
    {
        enresult(())
            .inspect(|_| {
                {
                    ::std::io::_print(format_args!("tag: {0}\n", "sub scope in while"));
                };
            })?;
    }
    Err(())
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
            };
        })
}
async fn skip_expr_async_await() -> Result<(), ()> {
    let _ = async {
        enresult(async {
                enresult(async {})
                    .inspect(|_| {
                        {
                            ::std::io::_print(
                                format_args!("tag: {0}\n", "sub scope in async"),
                            );
                        };
                    })?
                    .await;
                enresult(())
            })
            .inspect(|_| {
                {
                    ::std::io::_print(format_args!("tag: {0}\n", "sub scope in async"));
                };
            })?
            .await
            .inspect(|_| {
                {
                    ::std::io::_print(format_args!("tag: {0}\n", "sub scope in async"));
                };
            })?;
        enresult(async {
                enresult(())
                    .inspect(|_| {
                        {
                            ::std::io::_print(
                                format_args!("tag: {0}\n", "async & await"),
                            );
                        };
                    })?;
                enresult(())
            })?
            .await?;
        Err(())
            .inspect(|_| {
                {
                    ::std::io::_print(format_args!("tag: {0}\n", "async & await"));
                };
            })
    }
        .await?;
    Err(())
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
            };
        })
}
fn skip_last_ok() -> Result<(), ()> {
    let _: Result<(), ()> = {
        Err(())
            .inspect(|_| {
                {
                    ::std::io::_print(format_args!("tag: {0}\n", "last ok 1"));
                };
            })
    };
    let _: Result<(), ()> = { Err(()) };
    Err(())
}
