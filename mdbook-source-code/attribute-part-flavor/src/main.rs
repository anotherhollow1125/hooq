use hooq::hooq;

fn fallible<T>(val: T) -> Result<T, String> {
    Ok(val)
}

type MyResult = Result<(), String>;

#[hooq]
#[hooq::method(.inspect_err(|_| {
    let _x = $xxx;
    let _y = $yyy;
}))]
#[hooq::xxx = "from root"]
#[hooq::yyy = "from root"]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    fallible(())?;

    // Not hooked.
    let _ = || -> MyResult { fallible(()) };

    #[hooq::method = "my_flavor"]
    // Method will be changed.
    fallible(())?;

    #[hooq::result_types = "my_flavor"]
    // Hooked now.
    let _ = || -> MyResult { fallible(()) };

    #[hooq::bindings = "my_flavor"]
    // Bindings will be changed.
    fallible(())?;

    #[hooq::flavor = "my_flavor"]
    // All will be changed.
    fallible(())?;

    Ok(())
}
