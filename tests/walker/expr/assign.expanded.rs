use hooq::hooq;
fn hoge(v: usize) -> Result<usize, ()> {
    Ok(v * 2)
        .inspect_err(|e| {
            {
                ::std::io::_eprint(
                    format_args!(
                        "{0:?} @ path: {1}, line: {2}\n", e,
                        "<hooq_root>/tests/walker/expr/assign.rs", 5usize,
                    ),
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
    match x {
        tmp => {
            {
                ::std::io::_eprint(
                    format_args!(
                        "[{0}:{1}:{2}] {3} = {4:#?}\n",
                        "<hooq_root>/tests/walker/expr/assign.rs", 17u32,
                        5u32, "x", & tmp,
                    ),
                );
            };
            tmp
        }
    };
    #[allow(clippy::let_unit_value)]
    let _ = {
        x = hoge(2)
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
                        "<hooq_root>/tests/walker/expr/assign.rs", 26u32,
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
