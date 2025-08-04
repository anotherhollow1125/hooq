use hooq::hooq;
struct Hoge {
    field: u32,
}
fn hoge() -> Result<Hoge, ()> {
    Ok(Hoge { field: 10 })
        .inspect_err(|e| {
            let path = "<hooq_root>/tests/walker/expr/field.rs";
            let line = 9usize;
            {
                ::std::io::_eprint(
                    format_args!("{0:?} @ path: {1}, line: {2}\n", e, path, line),
                );
            };
        })
}
fn func() -> Result<(), ()> {
    let _ = hoge()
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
            };
        })?
        .field;
    let _ = {
        hoge()
            .inspect(|_| {
                {
                    ::std::io::_print(format_args!("tag: {0}\n", "inner"));
                };
            })?
    }
        .field;
    Ok(())
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
            };
        })
}
