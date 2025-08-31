use hooq::hooq;

#[hooq]
#[hooq::method(
    .inspect_err(|e| {
        ::log::error!("{:?} @ file: {}, line: {}", e, $file, $line);
    })
)]
fn a() -> Result<(), String> {
    Err("a".to_string())
}

#[hooq]
#[hooq::method(
    .inspect_err(|e| {
        ::log::error!("{:?} @ file: {}, line: {}", e, $file, $line);
    })
)]
fn b() -> Result<(), String> {
    a()?;
    Ok(())
}

#[hooq]
#[hooq::method(
    .inspect_err(|e| {
        ::log::error!("{:?} @ file: {}, line: {}", e, $file, $line);
    })
)]
fn c() -> Result<(), String> {
    b()?;
    Ok(())
}

#[hooq]
#[hooq::method(
    .inspect_err(|e| {
        ::log::error!("{:?} @ file: {}, line: {}", e, $file, $line);
    })
)]
fn main() -> Result<(), std::io::Error> {
    env_logger::init();

    c().map_err(std::io::Error::other)?;

    Ok(())
}
