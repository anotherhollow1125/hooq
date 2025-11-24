use hooq::hooq;

#[hooq]
#[hooq::method(.inspect_err(|_| { let _ = "specified @ root"; }))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    failable(())?;

    #[hooq::method(.inspect_err(|_| { let _ = "specified @ inner"; }))]
    failable(())?;

    Ok(())
}

fn failable<T>(val: T) -> Result<T, String> {
    Ok(val)
}
