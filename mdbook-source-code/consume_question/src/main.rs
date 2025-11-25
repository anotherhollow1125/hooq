use hooq::hooq;

fn failable<T>(val: T) -> Result<T, &'static str> {
    Ok(val)
}

#[hooq]
#[hooq::hook_targets("?")]
#[hooq::method(.unwrap()!)]
fn main() {
    failable(())?;
}
