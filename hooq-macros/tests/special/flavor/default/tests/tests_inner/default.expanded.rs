use std::fmt::Debug;
use hooq::hooq;
#[allow(unused)]
fn enresult<T: Debug>(val: T) -> Result<T, ()> {
    Ok(val)
}
#[allow(unused)]
fn func(flag: bool) -> Result<(), ()> {
    enresult(())
        .inspect_err(|e| {
            let path = "<hooq_root>/tests/tests_inner/default.rs";
            let line = 14usize;
            {
                ::std::io::_eprint(format_args!("[{0}:L{1}] {2:?}\n", path, line, e));
            };
        })?;
    if flag {
        return Err(())
            .inspect_err(|e| {
                let path = "<hooq_root>/tests/tests_inner/default.rs";
                let line = 17usize;
                {
                    ::std::io::_eprint(
                        format_args!("[{0}:L{1}] {2:?}\n", path, line, e),
                    );
                };
            });
    }
    Err(())
        .inspect_err(|e| {
            let path = "<hooq_root>/tests/tests_inner/default.rs";
            let line = 20usize;
            {
                ::std::io::_eprint(format_args!("[{0}:L{1}] {2:?}\n", path, line, e));
            };
        })
}
