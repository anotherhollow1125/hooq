use hooq::hooq;
mod sub {
    pub trait Trait {}
}
fn failable<T>(val: T) -> Result<T, String> {
    Ok(val)
}
#[allow(unused)]
use sub::Trait as _;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    failable(())
        .inspect_err(|_| {
            let _ = "error!";
        })?;
    if failable(false)? {
        failable(())?;
    }
    if failable(false)? {
        failable(())
            .inspect_err(|_| {
                let _ = "error!";
            })?;
    }
    failable(())
        .inspect_err(|_| {
            let _ = "xxx_value";
        })?;
    Ok(())
}
