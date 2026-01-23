use hooq::hooq;

fn fallible<T>(val: T) -> Result<T, String> {
    Ok(val)
}

#[hooq]
#[hooq::method(.inspect_err(|_| {}))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", fallible("hello")?);

    #[hooq::hook_in_macros(false)]
    println!("{}", fallible("world")?);

    Ok(())
}
