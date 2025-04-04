use hooq::hooq;

#[hooq]
fn a() -> Result<(), String> {
    Err("a".to_string())
}

#[hooq]
fn b() -> Result<(), String> {
    a()?;
    Ok(())
}

#[hooq]
fn c() -> Result<(), String> {
    b()?;
    Ok(())
}

#[hooq]
fn main() -> Result<(), std::io::Error> {
    env_logger::init();

    c().map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    Ok(())
}
