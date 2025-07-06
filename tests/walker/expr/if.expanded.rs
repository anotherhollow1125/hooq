use hooq::hooq;
fn hoge() -> Result<bool, ()> {
    Ok(true)
        .inspect_err(|e| {
            {
                ::std::io::_eprint(
                    format_args!(
                        "{0:?} @ file: {1}, line: {2}\n", e,
                        "/home/namn/workspace/hooq/tests/walker/expr/if.rs", 5usize,
                    ),
                );
            };
        })
}
fn func(flag: bool) -> Result<(), ()> {
    if hoge()
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0:?}\n", "(no tag)"));
            };
        })?
    {
        let _ = hoge()
            .inspect(|_| {
                {
                    ::std::io::_print(format_args!("tag: {0:?}\n", "(no tag)"));
                };
            })?;
        if flag {
            return Ok(())
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0:?}\n", "(no tag)"));
                    };
                });
        }
        if false {
            return hoge()
                .map(|_| ())
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0:?}\n", "(no tag)"));
                    };
                });
        }
        Ok(())
            .inspect(|_| {
                {
                    ::std::io::_print(format_args!("tag: {0:?}\n", "(no tag)"));
                };
            })
    } else {
        let _ = hoge()
            .inspect(|_| {
                {
                    ::std::io::_print(format_args!("tag: {0:?}\n", "(no tag)"));
                };
            })?;
        Err(())
            .inspect(|_| {
                {
                    ::std::io::_print(format_args!("tag: {0:?}\n", "(no tag)"));
                };
            })
    }
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0:?}\n", "(no tag)"));
            };
        })
}
