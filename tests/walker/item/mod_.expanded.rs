use hooq::hooq;
fn func() -> Result<(), ()> {
    mod tmp {
        #[allow(unused)]
        fn func() -> Result<(), ()> {
            Ok(())
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0:?}\n", "mod"));
                    };
                })
        }
    }
    Ok(())
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0:?}\n", "(no tag)"));
            };
        })
}
