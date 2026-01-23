use hooq::hooq;

#[hooq(eyre)]
pub fn a() -> eyre::Result<()> {
    let _n = { Some(10) }?;

    Err(eyre::eyre!("error!"))
}

#[hooq(eyre)]
pub fn b() -> eyre::Result<()> {
    a()?;

    Err(eyre::eyre!("error!"))
}

#[hooq(eyre)]
pub fn c() -> eyre::Result<()> {
    b()?;

    Err(eyre::eyre!("error!"))
}
