use hooq::hooq;

fn fallible<T>(val: T) -> Result<T, &'static str> {
    Ok(val)
}

#[hooq]
#[hooq::hook_targets("?")]
#[hooq::method(.unwrap()!)]
fn main() {
    fallible(())?;
}
