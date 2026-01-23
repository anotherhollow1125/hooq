use hooq::hooq;

fn fallible<T>(val: T) -> Result<T, String> {
    Ok(val)
}

#[hooq(my_match)]
#[allow(clippy::question_mark)]
fn main() -> Result<(), String> {
    let _ = fallible(42)?;

    Ok(())
}
