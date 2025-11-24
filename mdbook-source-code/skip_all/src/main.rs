use hooq::hooq;

#[hooq]
#[hooq::method(.inspect_err(|_| {}))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    failable(())?;

    #[hooq::skip_all]
    let f = || -> Option<()> {
        optional(())?; // When the hook is applied, an error occurs.

        Some(())
    };

    let _ = failable(f())?;

    Ok(())
}

fn failable<T>(val: T) -> Result<T, String> {
    Ok(val)
}

fn optional<T>(val: T) -> Option<T> {
    Some(val)
}
