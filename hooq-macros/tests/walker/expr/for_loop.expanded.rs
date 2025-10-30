use std::ops::RangeInclusive;
use hooq_macros::hooq;
fn range(end: usize) -> Result<RangeInclusive<usize>, ()> {
    Ok(0..=end)
}
fn hoge() -> Result<(), ()> {
    Err(())
        .inspect_err(|e| {
            let path = "<hooq_root>/tests/walker/expr/for_loop.rs";
            let line = 12usize;
            let col = 5usize;
            let expr = "Err(())";
            {
                ::std::io::_eprint(
                    format_args!(
                        "[{0}:{1}:{2}] {3:?}\n    {4}\n", path, line, col, e, expr
                    ),
                );
            };
        })
}
fn func() -> Result<(), ()> {
    for i in range(10)
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
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
                        ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
                    };
                });
        }
        if i > 50 {
            hoge()
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
                    };
                })?;
            return Err(())
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
                    };
                });
        }
        {
            ::std::io::_print(format_args!("{0} end\n", i));
        };
    }
    Err(())
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
            };
        })
}
