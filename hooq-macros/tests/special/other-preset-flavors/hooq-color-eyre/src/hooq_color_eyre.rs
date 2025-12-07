use color_eyre::eyre::{Result, eyre};
use hooq::hooq;

#[hooq(color_eyre)]
pub fn a() -> Result<()> {
    Err(eyre!("error!"))
}

#[hooq(color_eyre)]
pub fn b() -> Result<()> {
    a()?;

    Err(eyre!("error!"))
}

#[hooq(color_eyre)]
pub fn c() -> Result<()> {
    b()?;

    Err(eyre!("error!"))
}
