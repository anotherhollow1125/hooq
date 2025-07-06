use hooq::hooq;
#[allow(unused)]
fn hoge() -> Result<(), ()> {
    Ok(())
        .inspect_err(|e| {
            {
                ::std::io::_eprint(
                    format_args!(
                        "{0:?} @ file: {1}, line: {2}\n", e,
                        "/home/namn/workspace/hooq/tests/walker/expr/const.rs", 6usize,
                    ),
                );
            };
        })
}
fn func() -> Result<(), ()> {
    const _N: usize = const {
        fn _f() -> Result<(), ()> {
            hoge()
                .inspect(|_| {
                    {
                        ::std::io::_print(
                            format_args!("tag: {0:?}\n", "in const block"),
                        );
                    };
                })?;
            Ok(())
                .inspect(|_| {
                    {
                        ::std::io::_print(
                            format_args!("tag: {0:?}\n", "in const block"),
                        );
                    };
                })
        }
        10
    };
    Ok(())
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0:?}\n", "(no tag)"));
            };
        })
}
