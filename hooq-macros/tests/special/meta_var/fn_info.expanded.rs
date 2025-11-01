use hooq_macros::hooq;
fn enresult<T>(t: T) -> Result<T, ()> {
    Ok(t)
}
#[allow(dead_code)]
type Unit = ();
mod tmp {
    use std::sync::LazyLock;
    use super::{Unit, enresult};
    static _TMP: LazyLock<Result<(), ()>> = LazyLock::new(|| {
        enresult(())
            .inspect_err(|_| {
                {
                    ::std::io::_eprint(
                        format_args!(
                            "\nfn_name: {0}\nfn_sig:  {1}\n", "__closure_in_<unknown>__",
                            "| | {}"
                        ),
                    );
                };
            })?;
        let _ = || -> Result<(), ()> {
            enresult(())
                .inspect_err(|_| {
                    {
                        ::std::io::_eprint(
                            format_args!(
                                "\nfn_name: {0}\nfn_sig:  {1}\n",
                                "__closure_in_<unknown>__", "| | -> Result < (), () > {}"
                            ),
                        );
                    };
                })?;
            fn _inner() {
                let _ = || -> Result<Unit, Unit> {
                    enresult(())
                        .inspect_err(|_| {
                            {
                                ::std::io::_eprint(
                                    format_args!(
                                        "\nfn_name: {0}\nfn_sig:  {1}\n", "__closure_in__inner__",
                                        "| | -> Result < Unit, Unit > {}"
                                    ),
                                );
                            };
                        })?;
                    Ok(())
                };
            }
            enresult(())
                .inspect_err(|_| {
                    {
                        ::std::io::_eprint(
                            format_args!(
                                "\nfn_name: {0}\nfn_sig:  {1}\n",
                                "__closure_in_<unknown>__", "| | -> Result < (), () > {}"
                            ),
                        );
                    };
                })?;
            Ok(())
        };
        enresult(())
            .inspect_err(|_| {
                {
                    ::std::io::_eprint(
                        format_args!(
                            "\nfn_name: {0}\nfn_sig:  {1}\n", "__closure_in_<unknown>__",
                            "| | {}"
                        ),
                    );
                };
            })?;
        Ok(())
    });
}
fn func() -> Result<(), ()> {
    enresult(())
        .inspect_err(|_| {
            {
                ::std::io::_eprint(
                    format_args!(
                        "\nfn_name: {0}\nfn_sig:  {1}\n", "func",
                        "fn func() -> Result < (), () >"
                    ),
                );
            };
        })?;
    let _ = || -> Result<(), ()> {
        enresult(())
            .inspect_err(|_| {
                {
                    ::std::io::_eprint(
                        format_args!(
                            "\nfn_name: {0}\nfn_sig:  {1}\n", "__closure_in_func__",
                            "| | -> Result < (), () > {}"
                        ),
                    );
                };
            })?;
        fn _inner() -> Result<(), Unit> {
            enresult(())
                .inspect_err(|_| {
                    {
                        ::std::io::_eprint(
                            format_args!(
                                "\nfn_name: {0}\nfn_sig:  {1}\n", "_inner",
                                "fn _inner() -> Result < (), Unit >"
                            ),
                        );
                    };
                })?;
            let _ = || -> Result<Unit, Unit> {
                enresult(())
                    .inspect_err(|_| {
                        {
                            ::std::io::_eprint(
                                format_args!(
                                    "\nfn_name: {0}\nfn_sig:  {1}\n", "__closure_in__inner__",
                                    "| | -> Result < Unit, Unit > {}"
                                ),
                            );
                        };
                    })?;
                let _ = |(): Unit| -> Result<Unit, Unit> {
                    enresult(())
                        .inspect_err(|_| {
                            {
                                ::std::io::_eprint(
                                    format_args!(
                                        "\nfn_name: {0}\nfn_sig:  {1}\n", "__closure_in__inner__",
                                        "| () : Unit | -> Result < Unit, Unit > {}"
                                    ),
                                );
                            };
                        })?;
                    let _ = |(): Unit, (): Unit| -> Result<Unit, Unit> {
                        enresult(())
                            .inspect_err(|_| {
                                {
                                    ::std::io::_eprint(
                                        format_args!(
                                            "\nfn_name: {0}\nfn_sig:  {1}\n", "__closure_in__inner__",
                                            "| () : Unit, () : Unit | -> Result < Unit, Unit > {}"
                                        ),
                                    );
                                };
                            })?;
                        let _ = |((), (), ()): (Unit, Unit, Unit)| -> Result<Unit, Unit> {
                            enresult(())
                                .inspect_err(|_| {
                                    {
                                        ::std::io::_eprint(
                                            format_args!(
                                                "\nfn_name: {0}\nfn_sig:  {1}\n", "__closure_in__inner__",
                                                "| ((), (), ()) : (Unit, Unit, Unit) | -> Result < Unit, Unit > {}"
                                            ),
                                        );
                                    };
                                })?;
                            Ok(())
                        };
                        Ok(())
                    };
                    Ok(())
                };
                Ok(())
            };
            enresult(())
                .inspect_err(|_| {
                    {
                        ::std::io::_eprint(
                            format_args!(
                                "\nfn_name: {0}\nfn_sig:  {1}\n", "_inner",
                                "fn _inner() -> Result < (), Unit >"
                            ),
                        );
                    };
                })?;
            Ok(())
        }
        enresult(())
            .inspect_err(|_| {
                {
                    ::std::io::_eprint(
                        format_args!(
                            "\nfn_name: {0}\nfn_sig:  {1}\n", "__closure_in_func__",
                            "| | -> Result < (), () > {}"
                        ),
                    );
                };
            })?;
        Ok(())
    };
    enresult(())
        .inspect_err(|_| {
            {
                ::std::io::_eprint(
                    format_args!(
                        "\nfn_name: {0}\nfn_sig:  {1}\n", "func",
                        "fn func() -> Result < (), () >"
                    ),
                );
            };
        })?;
    Ok(())
}
