use hooq::hooq;

fn fallible<T>(val: T) -> Result<T, String> {
    Ok(val)
}

#[hooq]
#[hooq::method(.inspect_err(|_| { let _ = $xxx; }))]
#[hooq::xxx = 10]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    fallible(())?;

    #[hooq::binding(xxx = "in block")]
    {
        fallible(())?;

        #[hooq::var(xxx = 42)]
        fallible(())?;

        fallible(())?;
    }

    fallible(())?;

    Ok(())
}
