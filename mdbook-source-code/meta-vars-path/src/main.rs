use hooq::hooq;

fn failable<T>(val: T) -> Result<T, String> {
    Ok(val)
}

#[hooq]
#[hooq::method(.inspect_err(|_| {
    let _path = $path;
}))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    failable(())?;

    Ok(())
}
