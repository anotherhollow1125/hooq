use hooq_macros::hooq;
fn enresult<T>(v: T) -> Result<T, ()> {
    Ok(v)
        .inspect_err(|e| {
            let path = "<hooq_root>/tests/walker/stmt/macro.rs";
            let line = 5usize;
            {
                ::std::io::_eprint(
                    format_args!("{0:?} @ path: {1}, line: {2}\n", e, path, line),
                );
            };
        })
}
fn func() -> Result<(), ()> {
    {
        ::std::io::_print(
            format_args!(
                "{0}\n", enresult(10).inspect(| _ | { {
                ::std::io::_print(format_args!("tag: {0}, expr: {1}\n", "outer",
                "enresult(10)")); }; }) ?
            ),
        );
    };
    {
        ::std::io::_print(
            format_args!(
                "{0}\n", enresult(20).inspect(| _ | { {
                ::std::io::_print(format_args!("tag: {0}, expr: {1}\n", "inner",
                "enresult(20)")); }; }) ?
            ),
        );
    };
    Ok(())
        .inspect(|_| {
            {
                ::std::io::_print(
                    format_args!("tag: {0}, expr: {1}\n", "(no tag)", "Ok(())"),
                );
            };
        })
}
