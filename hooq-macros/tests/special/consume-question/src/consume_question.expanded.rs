use hooq::hooq;
fn enresult<T>(val: T) -> Result<T, String> {
    Ok(val)
}
pub fn func_unwrap() {
    enresult(()).unwrap();
}
#[allow(clippy::question_mark)]
pub fn func_match() -> Result<(), String> {
    match enresult(()) {
        Ok(v) => v,
        Err(e) => return Err(e),
    };
    Ok(())
}
