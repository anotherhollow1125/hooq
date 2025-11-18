use hooq_macros::hooq;
fn enresult<T>(t: T) -> Result<T, ()> {
    Ok(t)
}
mod tmp {
    use util_macros::id;
    use super::enresult;
    #[allow(unused)]
    macro_rules! tmp {
        () => {
            fn tmp_fn(flag : bool) -> Result < (), () > { if flag { enresult(()) ?;
            return Err(()); } Err(()) }
        };
    }
    pub fn outer() -> Result<(), ()> {
        enresult(())
            .inspect(|_| {
                {
                    ::std::io::_print(
                        format_args!("tag: {0}, expr: {1}\n", "outer", "enresult(()) ?"),
                    );
                };
            })?;
        Err(())
            .inspect(|_| {
                {
                    ::std::io::_print(
                        format_args!("tag: {0}, expr: {1}\n", "outer", "Err(())"),
                    );
                };
            })
    }
    const _CONST_VAL: usize = {
        fn _f() -> Result<(), ()> {
            enresult(())
                .inspect(|_| {
                    {
                        ::std::io::_print(
                            format_args!(
                                "tag: {0}, expr: {1}\n", "inner func", "enresult(()) ?"
                            ),
                        );
                    };
                })?;
            enresult(())
                .inspect(|_| {
                    {
                        ::std::io::_print(
                            format_args!(
                                "tag: {0}, expr: {1}\n", "inner func", "enresult(())"
                            ),
                        );
                    };
                })
        }
        10
    };
    pub fn inner() -> Result<(), ()> {
        enresult(())
            .inspect(|_| {
                {
                    ::std::io::_print(
                        format_args!("tag: {0}, expr: {1}\n", "inner", "enresult(()) ?"),
                    );
                };
            })?;
        Err(())
            .inspect(|_| {
                {
                    ::std::io::_print(
                        format_args!("tag: {0}, expr: {1}\n", "(no tag)", "Err(())"),
                    );
                };
            })
    }
}
