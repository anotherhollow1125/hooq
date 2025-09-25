use hooq_macros::hooq;
fn v() -> Result<Vec<usize>, ()> {
    Ok(<[_]>::into_vec(::alloc::boxed::box_new([1, 2, 3])))
}
fn hoge() -> Result<usize, ()> {
    Ok(1)
}
fn func() -> Result<(), ()> {
    let _ = v()
        .inspect_err(|e| {
            let path = "<hooq_root>/tests/walker/expr/index.rs";
            let line = 15usize;
            {
                ::std::io::_eprint(format_args!("[{0}:L{1}] {2:?}\n", path, line, e));
            };
        })?[hoge()
        .inspect_err(|e| {
            let path = "<hooq_root>/tests/walker/expr/index.rs";
            let line = 15usize;
            {
                ::std::io::_eprint(format_args!("[{0}:L{1}] {2:?}\n", path, line, e));
            };
        })?];
    Err(())
        .inspect_err(|e| {
            let path = "<hooq_root>/tests/walker/expr/index.rs";
            let line = 17usize;
            {
                ::std::io::_eprint(format_args!("[{0}:L{1}] {2:?}\n", path, line, e));
            };
        })
}
