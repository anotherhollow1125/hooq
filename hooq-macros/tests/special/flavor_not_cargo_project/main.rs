extern crate hooq;

use hooq::hooq;

#[hooq(custom)]
fn hoge(flag: bool) -> Result<(), Box<dyn std::error::Error>> {
    if flag {
        return Err("error".into());
    }

    Ok(())
}

#[hooq]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    hoge(false)?;

    Ok(())
}
