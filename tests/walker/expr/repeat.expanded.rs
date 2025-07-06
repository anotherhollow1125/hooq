use hooq::hooq;
fn hoge() -> Result<usize, ()> {
    Ok(5)
        .inspect_err(|e| {
            {
                ::std::io::_eprint(
                    format_args!(
                        "{0:?} @ file: {1}, line: {2}\n", e,
                        "/home/namn/workspace/hooq/tests/walker/expr/repeat.rs", 5usize,
                    ),
                );
            };
        })
}
fn func() -> Result<(), ()> {
    let _ = [hoge()
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
            };
        })?; {
        fn _f() -> Result<(), ()> {
            hoge()
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
                    };
                })?;
            Ok(())
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
                    };
                })
        }
        5
    }];
    Ok(())
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
            };
        })
}
