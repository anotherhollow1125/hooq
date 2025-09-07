use hooq::hooq;

#[hooq(log)]
fn a() -> Result<(), &'static str> {
    Err("error!")
}

#[hooq(log)]
fn b() -> Result<(), &'static str> {
    a()?;

    Ok(())
}

#[hooq(log)]
fn c() -> Result<(), &'static str> {
    b()?;

    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    env_logger::init();

    c().map_err(std::io::Error::other)?;

    Ok(())
}
