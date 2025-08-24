use hooq_macros::hooq;
fn v() -> Result<Vec<usize>, ()> {
    Ok(<[_]>::into_vec(::alloc::boxed::box_new([1, 2, 3])))
        .inspect_err(|e| {
            let path = "<hooq_root>/tests/walker/expr/index.rs";
            let line = 5usize;
            {
                ::std::io::_eprint(
                    format_args!("{0:?} @ path: {1}, line: {2}\n", e, path, line),
                );
            };
        })
}
fn hoge() -> Result<usize, ()> {
    Ok(1)
        .inspect_err(|e| {
            let path = "<hooq_root>/tests/walker/expr/index.rs";
            let line = 10usize;
            {
                ::std::io::_eprint(
                    format_args!("{0:?} @ path: {1}, line: {2}\n", e, path, line),
                );
            };
        })
}
fn func() -> Result<(), ()> {
    let _ = v()
        .inspect_err(|e| {
            let path = "<hooq_root>/tests/walker/expr/index.rs";
            let line = 15usize;
            {
                ::std::io::_eprint(
                    format_args!("{0:?} @ path: {1}, line: {2}\n", e, path, line),
                );
            };
        })?[hoge()
        .inspect_err(|e| {
            let path = "<hooq_root>/tests/walker/expr/index.rs";
            let line = 15usize;
            {
                ::std::io::_eprint(
                    format_args!("{0:?} @ path: {1}, line: {2}\n", e, path, line),
                );
            };
        })?];
    Ok(())
        .inspect_err(|e| {
            let path = "<hooq_root>/tests/walker/expr/index.rs";
            let line = 17usize;
            {
                ::std::io::_eprint(
                    format_args!("{0:?} @ path: {1}, line: {2}\n", e, path, line),
                );
            };
        })
}
