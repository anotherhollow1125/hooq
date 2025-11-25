use hooq::hooq;

fn failable<T>(val: T) -> Result<T, String> {
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
    failable(())?;

    // Not hooked.
    let _ = || -> MyResult { failable(()) };

    #[hooq::method = "my_flavor"]
    // Method will be changed.
    failable(())?;

    #[hooq::result_types = "my_flavor"]
    // Hooked now.
    let _ = || -> MyResult { failable(()) };

    #[hooq::bindings = "my_flavor"]
    // Bindings will be changed.
    failable(())?;

    #[hooq::flavor = "my_flavor"]
    // All will be changed.
    failable(())?;

    Ok(())
}
