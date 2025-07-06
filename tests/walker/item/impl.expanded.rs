use hooq::hooq;
fn func() -> Result<(), ()> {
    #[allow(unused)]
    struct S;
    impl S {
        #[allow(unused)]
        fn g() -> Result<(), ()> {
            Ok(())
                .inspect(|_| {
                    {
                        ::std::io::_print(
                            format_args!("tag: {0}\n", "impl related function"),
                        );
                    };
                })
        }
        #[allow(unused)]
        fn h() -> bool {
            true
        }
        #[allow(unused)]
        fn i(&self) -> Result<(), ()> {
            let res = Ok(());
            res.inspect(|_| {
                {
                    ::std::io::_print(format_args!("tag: {0}\n", "impl method"));
                };
            })
        }
        #[allow(unused)]
        fn j(&self) -> bool {
            true
        }
    }
    Ok(())
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
            };
        })
}
