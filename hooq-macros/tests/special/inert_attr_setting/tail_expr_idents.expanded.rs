use hooq_macros::hooq;
fn enresult<T>(val: T) -> Result<T, ()> {
    Ok(val)
}
fn func() -> Result<(), ()> {
    #[allow(non_snake_case)]
    let Right = || Result::<(), ()>::Ok(());
    #[allow(non_snake_case)]
    let Left = || Result::<(), ()>::Err(());
    enresult(())
        .inspect_err(|e| {
            let path = "<hooq_root>/tests/special/inert_attr_setting/tail_expr_idents.rs";
            let line = 21usize;
            {
                ::std::io::_eprint(format_args!("[{0}:L{1}] {2:?}\n", path, line, e));
            };
        })?;
    if enresult(false)
        .inspect_err(|e| {
            let path = "<hooq_root>/tests/special/inert_attr_setting/tail_expr_idents.rs";
            let line = 24usize;
            {
                ::std::io::_eprint(format_args!("[{0}:L{1}] {2:?}\n", path, line, e));
            };
        })?
    {
        return Ok(());
    }
    if enresult(false)
        .inspect_err(|e| {
            let path = "<hooq_root>/tests/special/inert_attr_setting/tail_expr_idents.rs";
            let line = 28usize;
            {
                ::std::io::_eprint(format_args!("[{0}:L{1}] {2:?}\n", path, line, e));
            };
        })?
    {
        return Err(());
    }
    if enresult(false)
        .inspect_err(|e| {
            let path = "<hooq_root>/tests/special/inert_attr_setting/tail_expr_idents.rs";
            let line = 33usize;
            {
                ::std::io::_eprint(format_args!("[{0}:L{1}] {2:?}\n", path, line, e));
            };
        })?
    {
        return Left()
            .inspect_err(|e| {
                let path = "<hooq_root>/tests/special/inert_attr_setting/tail_expr_idents.rs";
                let line = 34usize;
                {
                    ::std::io::_eprint(
                        format_args!("[{0}:L{1}] {2:?}\n", path, line, e),
                    );
                };
            });
    }
    let _ = || {
        Right()
            .inspect_err(|e| {
                let path = "<hooq_root>/tests/special/inert_attr_setting/tail_expr_idents.rs";
                let line = 37usize;
                {
                    ::std::io::_eprint(
                        format_args!("[{0}:L{1}] {2:?}\n", path, line, e),
                    );
                };
            })
    };
    Right()
        .inspect_err(|e| {
            let path = "<hooq_root>/tests/special/inert_attr_setting/tail_expr_idents.rs";
            let line = 39usize;
            {
                ::std::io::_eprint(format_args!("[{0}:L{1}] {2:?}\n", path, line, e));
            };
        })
}
