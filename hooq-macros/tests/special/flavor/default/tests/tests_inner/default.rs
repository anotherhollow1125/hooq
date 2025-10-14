use std::fmt::Debug;

use hooq::hooq;

#[hooq]
#[allow(unused)]
fn enresult<T: Debug>(val: T) -> Result<T, ()> {
    Ok(val)
}

#[hooq]
#[allow(unused)]
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
