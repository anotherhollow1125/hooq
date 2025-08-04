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
        let res = a()
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
        match res {
            tmp => {
                {
                    ::std::io::_eprint(
                        format_args!(
                            "[{0}:{1}:{2}] {3} = {4:#?}\n",
                            "<hooq_root>/tests/walker/stmt/expr.rs", 32u32,
                            9u32, "res", & tmp,
                        ),
                    );
                };
                tmp
            }
        };
    }
    let _ = g();
    f()
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0}\n", "tail expr"));
            };
        })
}
