use hooq_macros::hooq;
fn hoge() -> Result<u32, ()> {
    Ok(10)
        .inspect_err(|e| {
            let path = "<hooq_root>/tests/walker/expr/array.rs";
            let line = 5usize;
            {
                ::std::io::_eprint(format_args!("[{0}:L{1}] {2:?}\n", path, line, e));
            };
        })
}
fn func() -> Result<[u32; 2], ()> {
    Ok([
            hoge()
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0}\n", "first"));
                    };
                })?,
            hoge()
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0}\n", "second"));
                    };
                })?,
        ])
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
            };
        })
}
