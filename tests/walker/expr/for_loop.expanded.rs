use std::ops::RangeInclusive;
use hooq::hooq;
fn range(end: usize) -> Result<RangeInclusive<usize>, ()> {
    Ok(0..=end)
        .inspect_err(|e| {
            {
                ::std::io::_eprint(
                    format_args!(
                        "{0:?} @ file: {1}, line: {2}\n", e,
                        "/home/namn/workspace/hooq/tests/walker/expr/for_loop.rs",
                        7usize,
                    ),
                );
            };
        })
}
fn hoge() -> Result<(), ()> {
    Ok(())
        .inspect_err(|e| {
            {
                ::std::io::_eprint(
                    format_args!(
                        "{0:?} @ file: {1}, line: {2}\n", e,
                        "/home/namn/workspace/hooq/tests/walker/expr/for_loop.rs",
                        12usize,
                    ),
                );
            };
        })
}
fn func() -> Result<(), ()> {
    for i in range(10)
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0:?}\n", "(no tag)"));
            };
        })?
    {
        {
            ::std::io::_print(format_args!("{0} start\n", i));
        };
        if i > 100 {
            return hoge()
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0:?}\n", "(no tag)"));
                    };
                });
        }
        if i > 50 {
            hoge()
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0:?}\n", "(no tag)"));
                    };
                })?;
            return Err(())
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0:?}\n", "(no tag)"));
                    };
                });
        }
        {
            ::std::io::_print(format_args!("{0} end\n", i));
        };
    }
    Ok(())
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0:?}\n", "(no tag)"));
            };
        })
}
