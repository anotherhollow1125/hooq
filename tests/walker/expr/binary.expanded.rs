use hooq::hooq;
fn b() -> Result<u32, ()> {
    Ok(10)
        .inspect_err(|e| {
            let path = "<hooq_root>/tests/walker/expr/binary.rs";
            let line = 5usize;
            {
                ::std::io::_eprint(
                    format_args!("{0:?} @ path: {1}, line: {2}\n", e, path, line),
                );
            };
        })
}
fn c() -> Result<u32, ()> {
    Ok(20)
        .inspect_err(|e| {
            let path = "<hooq_root>/tests/walker/expr/binary.rs";
            let line = 10usize;
            {
                ::std::io::_eprint(
                    format_args!("{0:?} @ path: {1}, line: {2}\n", e, path, line),
                );
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
