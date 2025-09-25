use hooq_macros::hooq;
fn func(_: ()) -> Result<(), ()> {
    Err(())
        .inspect_err(|e| {
            let path = "<hooq_root>/tests/special/nested.rs";
            let line = 5usize;
            {
                ::std::io::_eprint(format_args!("[{0}:L{1}] {2:?}\n", path, line, e));
            };
        })
}
fn nested() -> Result<(), ()> {
    func(
            func(
                    func(
                            func(())
                                .inspect(|_| {
                                    let path = "<hooq_root>/tests/special/nested.rs";
                                    let line = 20usize;
                                    let expr = "func(())";
                                    {
                                        ::std::io::_eprint(
                                            format_args!(
                                                "nested @\npath: {0},\nline: {1}\nexpr: {2}\n", path, line,
                                                expr
                                            ),
                                        );
                                    };
                                })?,
                        )
                        .inspect(|_| {
                            let path = "<hooq_root>/tests/special/nested.rs";
                            let line = 20usize;
                            let expr = "func(func(()) ?)";
                            {
                                ::std::io::_eprint(
                                    format_args!(
                                        "nested @\npath: {0},\nline: {1}\nexpr: {2}\n", path, line,
                                        expr
                                    ),
                                );
                            };
                        })?,
                )
                .inspect(|_| {
                    let path = "<hooq_root>/tests/special/nested.rs";
                    let line = 20usize;
                    let expr = "func(func(func(()) ?) ?)";
                    {
                        ::std::io::_eprint(
                            format_args!(
                                "nested @\npath: {0},\nline: {1}\nexpr: {2}\n", path, line,
                                expr
                            ),
                        );
                    };
                })?,
        )
        .inspect(|_| {
            let path = "<hooq_root>/tests/special/nested.rs";
            let line = 20usize;
            let expr = "func(func(func(func(()) ?) ?) ?)";
            {
                ::std::io::_eprint(
                    format_args!(
                        "nested @\npath: {0},\nline: {1}\nexpr: {2}\n", path, line, expr
                    ),
                );
            };
        })?;
    Err(())
        .inspect(|_| {
            let path = "<hooq_root>/tests/special/nested.rs";
            let line = 21usize;
            let expr = "Err(())";
            {
                ::std::io::_eprint(
                    format_args!(
                        "nested @\npath: {0},\nline: {1}\nexpr: {2}\n", path, line, expr
                    ),
                );
            };
        })
}
