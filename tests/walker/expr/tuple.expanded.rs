use hooq::hooq;
fn hoge() -> Result<(), ()> {
    Ok(())
        .inspect_err(|e| {
            {
                ::std::io::_eprint(
                    format_args!(
                        "{0:?} @ file: {1}, line: {2}\n", e,
                        "/home/namn/workspace/hooq/tests/walker/expr/tuple.rs", 5usize,
                    ),
                );
            };
        })
}
fn func(flag: bool) -> Result<(), ()> {
    let _: ((), usize, Result<(), ()>) = (
        hoge()
            .inspect(|_| {
                {
                    ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
                };
            })?,
        {
            if !flag {
                return Err(())
                    .inspect(|_| {
                        {
                            ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
                        };
                    });
            }
            10
        },
        {
            if flag {
                return hoge()
                    .inspect(|_| {
                        {
                            ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
                        };
                    });
            }
            Ok(())
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
                    };
                })
        },
    );
    Ok(())
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
            };
        })
}
