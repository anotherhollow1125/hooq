use hooq::hooq;
struct Strct;
impl Strct {
    fn method(&self, _val1: usize, _val2: usize) -> Result<(), ()> {
        Ok(())
            .inspect_err(|e| {
                {
                    ::std::io::_eprint(
                        format_args!(
                            "{0:?} @ file: {1}, line: {2}\n", e,
                            "/home/namn/workspace/hooq/tests/walker/expr/method_call.rs",
                            8usize,
                        ),
                    );
                };
            })
    }
}
fn get_strct() -> Result<Strct, ()> {
    Ok(Strct)
        .inspect_err(|e| {
            {
                ::std::io::_eprint(
                    format_args!(
                        "{0:?} @ file: {1}, line: {2}\n", e,
                        "/home/namn/workspace/hooq/tests/walker/expr/method_call.rs",
                        14usize,
                    ),
                );
            };
        })
}
fn get_val() -> Result<usize, ()> {
    Ok(10)
        .inspect_err(|e| {
            {
                ::std::io::_eprint(
                    format_args!(
                        "{0:?} @ file: {1}, line: {2}\n", e,
                        "/home/namn/workspace/hooq/tests/walker/expr/method_call.rs",
                        19usize,
                    ),
                );
            };
        })
}
fn func() -> Result<(), ()> {
    get_strct()
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0:?}\n", "(no tag)"));
            };
        })?
        .method(
            get_val()
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0:?}\n", "(no tag)"));
                    };
                })?,
            get_val()
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0:?}\n", "(no tag)"));
                    };
                })?,
        )
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0:?}\n", "(no tag)"));
            };
        })?;
    Ok(())
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0:?}\n", "(no tag)"));
            };
        })
}
