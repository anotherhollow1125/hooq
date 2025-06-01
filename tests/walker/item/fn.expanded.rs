use hooq::hooq;
fn func() -> Result<(), ()> {
    fn f() -> Result<(), ()> {
        fn g() -> Result<(), ()> {
            Err(())
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0:?}\n", "func inner 1"));
                    };
                })
        }
        fn h() -> bool {
            {
                ::std::io::_print(format_args!("Hello, world!\n"));
            };
            true
        }
        let _ = g();
        let _ = h();
        Ok(())
            .inspect(|_| {
                {
                    ::std::io::_print(format_args!("tag: {0:?}\n", "func"));
                };
            })
    }
    f()
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0:?}\n", "(no tag)"));
            };
        })?;
    Ok(())
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0:?}\n", "(no tag)"));
            };
        })
}
