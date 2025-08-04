use hooq::hooq;
fn b() -> Result<u32, ()> {
    Ok(10)
        .inspect_err(|e| {
            {
                ::std::io::_eprint(
                    format_args!(
                        "{0:?} @ path: {1}, line: {2}\n", e,
                        "<hooq_root>/tests/walker/expr/binary.rs", 5usize
                    ),
                );
            };
        })
}
fn c() -> Result<u32, ()> {
    Ok(20)
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("(# \u{ff9f}Д\u{ff9f})\n"));
            };
        })
}
fn func() -> Result<u32, ()> {
    let mut a = b()
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0}\n", "add"));
            };
        })?
        + c()
            .inspect(|_| {
                {
                    ::std::io::_print(format_args!("tag: {0}\n", "add"));
                };
            })?;
    {
        a
            += b()
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0}\n", "add_assign"));
                    };
                })?;
    }
    Ok(
            a
                + b()
                    .inspect(|_| {
                        {
                            ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
                        };
                    })?
                + c()
                    .inspect(|_| {
                        {
                            ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
                        };
                    })?,
        )
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
            };
        })
}
