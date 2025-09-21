use hooq::hooq;

#[hooq(anyhow)]
pub fn a() -> anyhow::Result<()> {
    Err(anyhow::anyhow!("error!"))
}

#[hooq(anyhow)]
pub fn b() -> anyhow::Result<()> {
    a()?;

    Err(anyhow::anyhow!("error!"))
}

#[hooq(anyhow)]
pub fn c() -> anyhow::Result<()> {
    b()?;

    Err(anyhow::anyhow!("error!"))
}
