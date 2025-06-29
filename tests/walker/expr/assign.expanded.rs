use hooq::hooq;
fn hoge() -> Result<usize, ()> {
    Ok(10)
        .inspect_err(|e| {
            {
                ::std::io::_eprint(
                    format_args!(
                        "{0:?} @ file: {1}, line: {2}\n", e,
                        "/home/namn/workspace/hooq/tests/walker/expr/assign.rs", 5usize,
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
    match x {
        tmp => {
            {
                ::std::io::_eprint(
                    format_args!(
                        "[{0}:{1}:{2}] {3} = {4:#?}\n",
                        "/home/namn/workspace/hooq/tests/walker/expr/assign.rs", 22u32,
                        5u32, "x", & tmp,
                    ),
                );
            };
            tmp
        }
    };
    Ok(())
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("{0}\n", "(no tag)"));
            };
        })
}
