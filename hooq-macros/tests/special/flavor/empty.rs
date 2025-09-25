use hooq_macros::hooq;

#[hooq(flavor = "empty")]
fn enresult<T>(val: T) -> Result<T, ()> {
    Ok(val)
}

#[hooq(empty)]
fn func(flag: bool) -> Result<(), ()> {
    enresult(())?;

    if flag {
        return Err(());
    }

    Err(())
}

#[test]
fn test() {
    func(false).unwrap_err();
}
