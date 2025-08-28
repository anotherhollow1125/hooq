use hooq_macros::hooq;
struct Strct;
impl Strct {
    fn method(&self, _val1: usize, _val2: usize) -> Result<(), ()> {
        Ok(())
            .inspect_err(|e| {
                let path = "<hooq_root>/tests/walker/expr/method_call.rs";
                let line = 8usize;
                {
                    ::std::io::_eprint(
                        format_args!("[{0}:L{1}] {2:?}\n", path, line, e),
                    );
                };
            })
    }
}
fn get_strct() -> Result<Strct, ()> {
    Ok(Strct)
        .inspect_err(|e| {
            let path = "<hooq_root>/tests/walker/expr/method_call.rs";
            let line = 14usize;
            {
                ::std::io::_eprint(format_args!("[{0}:L{1}] {2:?}\n", path, line, e));
            };
        })
}
fn get_val() -> Result<usize, ()> {
    Ok(10)
        .inspect_err(|e| {
            let path = "<hooq_root>/tests/walker/expr/method_call.rs";
            let line = 19usize;
            {
                ::std::io::_eprint(format_args!("[{0}:L{1}] {2:?}\n", path, line, e));
            };
        })
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
    Ok(())
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
            };
        })
}
