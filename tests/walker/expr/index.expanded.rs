use hooq::hooq;
fn v() -> Result<Vec<usize>, ()> {
    Ok(<[_]>::into_vec(::alloc::boxed::box_new([1, 2, 3])))
        .inspect_err(|e| {
            {
                ::std::io::_eprint(
                    format_args!(
                        "{0:?} @ path: {1}, line: {2}\n", e,
                        "<hooq_root>/tests/walker/expr/index.rs", 5usize,
                    ),
                );
            };
        })
}
fn hoge() -> Result<usize, ()> {
    Ok(1)
        .inspect_err(|e| {
            {
                ::std::io::_eprint(
                    format_args!(
                        "{0:?} @ path: {1}, line: {2}\n", e,
                        "<hooq_root>/tests/walker/expr/index.rs", 10usize,
                    ),
                );
            };
        })
}
fn func() -> Result<(), ()> {
    let _ = v()
        .inspect_err(|e| {
            {
                ::std::io::_eprint(
                    format_args!(
                        "{0:?} @ path: {1}, line: {2}\n", e,
                        "<hooq_root>/tests/walker/expr/index.rs", 15usize,
                    ),
                );
            };
        })?[hoge()
        .inspect_err(|e| {
            {
                ::std::io::_eprint(
                    format_args!(
                        "{0:?} @ path: {1}, line: {2}\n", e,
                        "<hooq_root>/tests/walker/expr/index.rs", 15usize,
                    ),
                );
            };
        })?];
    Ok(())
        .inspect_err(|e| {
            {
                ::std::io::_eprint(
                    format_args!(
                        "{0:?} @ path: {1}, line: {2}\n", e,
                        "<hooq_root>/tests/walker/expr/index.rs", 17usize,
                    ),
                );
            };
        })
}
