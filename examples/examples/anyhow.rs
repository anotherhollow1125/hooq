use hooq::hooq;

#[hooq(anyhow)]
fn a() -> anyhow::Result<()> {
    Err(anyhow::anyhow!("error!"))
}

#[hooq(anyhow)]
fn b() -> anyhow::Result<()> {
    a()?;

    Result::<(), anyhow::Error>::Ok(())
}

#[hooq(anyhow)]
fn c() -> anyhow::Result<()> {
    b()?;

    Result::<(), anyhow::Error>::Ok(())
}

fn main() -> anyhow::Result<()> {
    c()?;

    Ok(())
}
