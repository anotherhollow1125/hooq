use hooq_macros::hooq;
fn enresult<T>(t: T) -> Result<T, ()> {
    Ok(t)
}
fn func() -> Result<(), ()> {
    enresult(())
        .inspect_err(|_| {
            {
                ::std::io::_print(
                    format_args!("line!(): {0} $line: {1}\n", 7u32, 12usize),
                );
            };
        })?;
    Err(())
        .inspect_err(|_| {
            {
                ::std::io::_print(
                    format_args!("line!(): {0} $line: {1}\n", 7u32, 14usize),
                );
            };
        })
}
