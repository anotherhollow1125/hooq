use hooq::hooq;

fn fallible<T>(val: T) -> Result<T, String> {
    Ok(val)
}

#[hooq]
#[hooq::string = "hello!"]
#[hooq::method(.inspect_err(|_| {
    let _hooq_meta = $hooq_meta;
}))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    fallible(())?;

    Ok(())
}
