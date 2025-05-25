use hooq::hooq;
#[cfg(all())]
fn func() -> Result<(), ()> {
    #[cfg(all())]
    fn f() -> Result<(), ()> {
        #[cfg(all())]
        fn g() -> Result<(), ()> {
            Err(())
                .map(|v| {
                    {
                        ::std::io::_print(format_args!("tag: {0:?}\n", "func inner 1"));
                    };
                    v
                })
        }
        #[cfg(all())]
        fn h() -> bool {
            {
                ::std::io::_print(format_args!("Hello, world!\n"));
            };
            true
        }
        let _ = g();
        let _ = h();
        Ok(())
            .map(|v| {
                {
                    ::std::io::_print(format_args!("tag: {0:?}\n", "func"));
                };
                v
            })
    }
    f()
        .map(|v| {
            {
                ::std::io::_print(format_args!("tag: {0:?}\n", "(no tag)"));
            };
            v
        })?;
    Ok(())
        .map(|v| {
            {
                ::std::io::_print(format_args!("tag: {0:?}\n", "(no tag)"));
            };
            v
        })
}
fn main() {
    func().unwrap();
}
