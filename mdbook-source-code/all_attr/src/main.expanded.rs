use hooq::hooq;
mod sub {
    pub trait Trait {}
}
fn fallible<T>(val: T) -> Result<T, String> {
    Ok(val)
}
#[allow(unused)]
use sub::Trait as _;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    fallible(())
        .inspect_err(|_| {
            let _ = "error!";
        })?;
    if fallible(false)? {
        fallible(())?;
    }
    if fallible(false)? {
        fallible(())
            .inspect_err(|_| {
                let _ = "error!";
            })?;
    }
    fallible(())
        .inspect_err(|_| {
            let _ = "xxx_value";
        })?;
    Ok(())
}
