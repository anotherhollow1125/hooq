use hooq::hooq;
fn hoge() -> Result<usize, ()> {
    Ok(10)
        .inspect_err(|e| {
            {
                ::std::io::_eprint(
                    format_args!(
                        "{0:?} @ file: {1}\n", e,
                        "/home/namn/workspace/hooq/tests/walker/expr/assign.rs",
                    ),
                );
            };
        })
}
fn func() -> Result<(), ()> {
    let x;
    #[allow(clippy::let_unit_value)]
    let _ = {
        x = hoge()
            .inspect(|_| {
                {
                    ::std::io::_print(format_args!("{0}\n", "inner"));
                };
            })?
    };
    {
        ::std::io::_print(format_args!("x: {0}\n", x));
    };
    Ok(())
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("{0}\n", "(no tag)"));
            };
        })
}
