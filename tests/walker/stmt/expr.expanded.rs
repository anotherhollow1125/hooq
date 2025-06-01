use hooq::hooq;
fn a() -> Result<bool, ()> {
    Ok(true)
}
fn b() -> Result<bool, ()> {
    Ok(false)
}
fn func() -> Result<(), ()> {
    let f = || {
        Ok(())
            .inspect(|_| {
                {
                    ::std::io::_print(format_args!("tag: {0:?}\n", "inner"));
                };
            })
    };
    let g = || {
        {
            ::std::io::_print(format_args!("beep\n"));
        };
        true
    };
    {
        let res = a()
            .inspect(|_| {
                {
                    ::std::io::_print(
                        format_args!("tag: {0:?}\n", "in eq binary ops exprs"),
                    );
                };
            })?
            == b()
                .inspect(|_| {
                    {
                        ::std::io::_print(
                            format_args!("tag: {0:?}\n", "in eq binary ops exprs"),
                        );
                    };
                })?;
        {
            ::std::io::_print(format_args!("{0:?}\n", res));
        };
    }
    let _ = g();
    f()
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0:?}\n", "tail expr"));
            };
        })
}
