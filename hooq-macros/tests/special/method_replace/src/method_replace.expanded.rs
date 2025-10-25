#![allow(clippy::result_unit_err)]
use hooq::hooq;
fn enresult<T>(val: T) -> Result<T, ()> {
    Ok(val)
}
pub fn func_unwrap() -> Result<(), ()> {
    if Ok(enresult(false).unwrap())? {
        return Ok(Err::<(), ()>(()).unwrap());
    }
    Ok(enresult(()).unwrap())?;
    Ok(())
}
pub fn func_overridden() -> Result<(), ()> {
    Ok(())?;
    Ok(())
}
