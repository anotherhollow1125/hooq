use hooq::hooq;

fn failable<T>(val: T) -> Result<T, String> {
    Ok(val)
}

#[hooq]
#[hooq::method(.inspect_err(|_| {}))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", failable("hello")?);

    #[hooq::hook_in_macros(false)]
    println!("{}", failable("world")?);

    Ok(())
}
