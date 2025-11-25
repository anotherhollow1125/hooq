use hooq::hooq;

#[hooq(my_flavor)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    func()?;

    Ok(())
}

#[hooq(my_flavor::sub_flavor)]
fn failable<T>(val: T) -> Result<T, String> {
    Ok(val)
}

#[hooq(flavor = "my_flavor.sub_flavor")]
fn func() -> Result<(), Box<dyn std::error::Error>> {
    failable(())?;

    Ok(())
}
