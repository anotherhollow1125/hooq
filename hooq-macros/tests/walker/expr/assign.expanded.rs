use hooq_macros::hooq;
fn hoge(v: usize) -> Result<usize, ()> {
    Ok(v * 2)
        .inspect_err(|e| {
            let path = "<hooq_root>/tests/walker/expr/assign.rs";
            let line = 5usize;
            {
                ::std::io::_eprint(
                    format_args!("{0:?} @ path: {1}, line: {2}\n", e, path, line),
                );
            };
        })
}
fn func() -> Result<(), ()> {
    let mut x;
    x = hoge(1)
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("{0}\n", "(no tag)"));
            };
        })?;
    let _ = x;
    #[allow(clippy::let_unit_value)]
    let _ = {
        x = hoge(2)
            .inspect(|_| {
                {
                    ::std::io::_print(format_args!("{0}\n", "inner"));
                };
            })?
    };
    let _ = x;
    Ok(())
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("{0}\n", "(no tag)"));
            };
        })
}
