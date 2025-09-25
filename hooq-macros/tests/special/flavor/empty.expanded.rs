use hooq_macros::hooq;
fn enresult<T>(val: T) -> Result<T, ()> {
    Ok(val)
}
fn func(flag: bool) -> Result<(), ()> {
    enresult(())?;
    if flag {
        return Err(());
    }
    Err(())
}
