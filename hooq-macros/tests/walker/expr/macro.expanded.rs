use hooq_macros::hooq;
fn enresult<T>(v: T) -> Result<T, ()> {
    Ok(v)
}
fn func() -> Result<(), ()> {
    let _ = ::alloc::vec::from_elem(
        enresult(10)
            .inspect(|_| {
                {
                    ::std::io::_print(
                        format_args!("tag: {0}, expr: {1}\n", "outer", "enresult(10) ?"),
                    );
                };
            })?,
        enresult(2)
            .inspect(|_| {
                {
                    ::std::io::_print(
                        format_args!("tag: {0}, expr: {1}\n", "outer", "enresult(2) ?"),
                    );
                };
            })?,
    );
    let _ = <[_]>::into_vec(
        ::alloc::boxed::box_new([
            enresult(10)
                .inspect(|_| {
                    {
                        ::std::io::_print(
                            format_args!(
                                "tag: {0}, expr: {1}\n", "inner 1", "enresult(10) ?"
                            ),
                        );
                    };
                })?,
            enresult(20)
                .inspect(|_| {
                    {
                        ::std::io::_print(
                            format_args!(
                                "tag: {0}, expr: {1}\n", "inner 2", "enresult(20) ?"
                            ),
                        );
                    };
                })?,
            enresult(30)
                .inspect(|_| {
                    {
                        ::std::io::_print(
                            format_args!(
                                "tag: {0}, expr: {1}\n", "inner 3", "enresult(30) ?"
                            ),
                        );
                    };
                })?,
        ]),
    );
    Err(())
        .inspect(|_| {
            {
                ::std::io::_print(
                    format_args!("tag: {0}, expr: {1}\n", "(no tag)", "Err(())"),
                );
            };
        })
}
