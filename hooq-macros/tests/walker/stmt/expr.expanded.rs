use hooq_macros::hooq;
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
                    ::std::io::_print(format_args!("tag: {0}\n", "inner"));
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
        let _ = a()
            .inspect(|_| {
                {
                    ::std::io::_print(
                        format_args!("tag: {0}\n", "in eq binary ops exprs"),
                    );
                };
            })?
            == b()
                .inspect(|_| {
                    {
                        ::std::io::_print(
                            format_args!("tag: {0}\n", "in eq binary ops exprs"),
                        );
                    };
                })?;
    }
    let _ = g();
    f()
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0}\n", "tail expr"));
            };
        })
}
