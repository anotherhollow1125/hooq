use hooq::hooq;

fn failable<T>(val: T) -> Result<T, String> {
    Ok(val)
}

#[hooq]
#[hooq::method(.inspect_err(|_| { let _ = $xxx; }))]
#[hooq::xxx = 10]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    failable(())?;

    #[hooq::binding(xxx = "in block")]
    {
        failable(())?;

        #[hooq::var(xxx = 42)]
        failable(())?;

        failable(())?;
    }

    failable(())?;

    Ok(())
}
