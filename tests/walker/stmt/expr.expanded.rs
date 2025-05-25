use hooq::hooq;
fn a() -> Result<bool, ()> {
    true
}
fn b() -> Result<bool, ()> {
    false
}
#[cfg(all())]
fn func() -> Result<(), ()> {
    #[cfg(all())]
    let f = || {
        #[cfg(all())]
        Ok(())
            .map(|v| {
                {
                    ::std::io::_print(format_args!("tag: {0:?}\n", "inner"));
                };
                v
            })
    };
    #[cfg(all())]
    let g = || {
        {
            ::std::io::_print(format_args!("beep\n"));
        };
        true
    };
    #[cfg(all())]
    {
        let res = a()
            .map(|v| {
                {
                    ::std::io::_print(
                        format_args!("tag: {0:?}\n", "in eq binary ops exprs"),
                    );
                };
                v
            })?
            == b()
                .map(|v| {
                    {
                        ::std::io::_print(
                            format_args!("tag: {0:?}\n", "in eq binary ops exprs"),
                        );
                    };
                    v
                })?;
        {
            ::std::io::_print(format_args!("{0:?}\n", res));
        };
    }
    let _ = g();
    #[cfg(all())]
    f()
        .map(|v| {
            {
                ::std::io::_print(format_args!("tag: {0:?}\n", "tail expr"));
            };
            v
        })
}
fn main() {
    func().unwrap();
}
