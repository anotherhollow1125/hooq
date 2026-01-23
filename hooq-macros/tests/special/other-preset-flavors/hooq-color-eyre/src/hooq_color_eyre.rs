use color_eyre::eyre::{Result, eyre};
use hooq::hooq;

#[hooq(color_eyre)]
pub fn a() -> Result<()> {
    let _n = { Some(10) }?;

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
