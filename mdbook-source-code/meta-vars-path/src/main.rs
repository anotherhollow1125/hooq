use hooq::hooq;

fn fallible<T>(val: T) -> Result<T, String> {
    Ok(val)
}

#[hooq]
#[hooq::method(.inspect_err(|_| {
    let _path = $path;
}))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    fallible(())?;

    Ok(())
}
