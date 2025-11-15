use hooq_macros::hooq;
struct Strct;
impl Strct {
    fn method(&self, _val1: usize, _val2: usize) -> Result<(), ()> {
        Err(())
            .inspect_err(|e| {
                let path = "<hooq_root>/tests/walker/expr/method_call.rs";
                let line = 8usize;
                let col = 9usize;
                let expr = "   8>    Err(())\n    |";
                {
                    ::std::io::_eprint(
                        format_args!(
                            "[{0}:{1}:{2}] {3:?}\n{4}\n", path, line, col, e, expr
                        ),
                    );
                };
            })
    }
}
fn get_strct() -> Result<Strct, ()> {
    Ok(Strct)
}
fn get_val() -> Result<usize, ()> {
    Ok(10)
}
fn func() -> Result<(), ()> {
    get_strct()
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
            };
        })?
        .method(
            get_val()
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
                    };
                })?,
            get_val()
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
                    };
                })?,
        )
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
            };
        })?;
    Err(())
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
            };
        })
}
