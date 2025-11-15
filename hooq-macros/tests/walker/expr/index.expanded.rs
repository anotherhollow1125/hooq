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
            let col = 16usize;
            let expr = "  15|             v()?\n    |";
            {
                ::std::io::_eprint(
                    format_args!("[{0}:{1}:{2}] {3:?}\n{4}\n", path, line, col, e, expr),
                );
            };
        })?[hoge()
        .inspect_err(|e| {
            let path = "<hooq_root>/tests/walker/expr/index.rs";
            let line = 15usize;
            let col = 24usize;
            let expr = "  15|                  hoge()?\n    |";
            {
                ::std::io::_eprint(
                    format_args!("[{0}:{1}:{2}] {3:?}\n{4}\n", path, line, col, e, expr),
                );
            };
        })?];
    Err(())
        .inspect_err(|e| {
            let path = "<hooq_root>/tests/walker/expr/index.rs";
            let line = 17usize;
            let col = 5usize;
            let expr = "  17|     Err(())\n    |";
            {
                ::std::io::_eprint(
                    format_args!("[{0}:{1}:{2}] {3:?}\n{4}\n", path, line, col, e, expr),
                );
            };
        })
}
