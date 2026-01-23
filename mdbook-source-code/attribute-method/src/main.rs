use hooq::hooq;

#[hooq]
#[hooq::method(.inspect_err(|_| { let _ = "specified @ root"; }))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    fallible(())?;

    #[hooq::method(.inspect_err(|_| { let _ = "specified @ inner"; }))]
    fallible(())?;

    Ok(())
}

fn fallible<T>(val: T) -> Result<T, String> {
    Ok(val)
}
