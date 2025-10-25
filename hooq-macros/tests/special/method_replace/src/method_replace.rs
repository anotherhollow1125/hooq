#![allow(clippy::result_unit_err)]

use hooq::hooq;

#[hooq]
fn enresult<T>(val: T) -> Result<T, ()> {
    Ok(val)
}

#[hooq]
#[hooq::method(Ok($expr.unwrap()))]
pub fn func_unwrap() -> Result<(), ()> {
    if enresult(false)? {
        return Err::<(), ()>(());
    }

    enresult(())?;

    Ok(())
}

#[hooq]
#[hooq::method(Ok(()))]
pub fn func_overridden() -> Result<(), ()> {
    enresult(())?;

    Ok(())
}

#[hooq(custom)]
pub fn func_with_debug() -> Result<(), ()> {
    if enresult(false)? {
        return Err::<(), ()>(());
    }

    enresult(())?;

    Ok(())
}
