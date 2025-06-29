use hooq::hooq;
fn hoge() -> Result<u32, ()> {
    Ok(10)
        .inspect_err(|e| {
            {
                ::std::io::_eprint(
                    format_args!(
                        "{0:?} @ file: {1}, line: {2}\n", e,
                        "/home/namn/workspace/hooq/tests/walker/expr/array.rs", 5usize,
                    ),
                );
            };
        })
}
fn func() -> Result<[u32; 2], ()> {
    Ok([
            hoge()
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0:?}\n", "first"));
                    };
                })?,
            hoge()
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0:?}\n", "second"));
                    };
                })?,
        ])
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0:?}\n", "(no tag)"));
            };
        })
}
