use hooq::hooq;

fn enresult<T>(val: T) -> Result<T, String> {
    Ok(val)
}

#[hooq]
#[hooq::method(.unwrap()!)]
pub fn func_unwrap() {
    enresult(())?;
}

#[hooq]
#[hooq::method(match $expr {
    Ok(v) => v,
    Err(e) => return Err(e),
}!)]
#[hooq::hook_targets("?")]
#[allow(clippy::question_mark)]
pub fn func_match() -> Result<(), String> {
    enresult(())?;

    Ok(())
}
