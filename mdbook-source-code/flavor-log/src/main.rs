use hooq::hooq;

#[hooq(log)] // default log level is Error
fn a() -> Result<(), String> {
    Err("An error occurred in function a".to_string())
}

#[hooq(log::warn)]
fn b() -> Result<(), String> {
    a()?;

    Ok(())
}

#[hooq(log::info)]
fn c(flag: bool) -> Result<(), String> {
    if flag {
        #[hooq::level = log::Level::Error]
        b()?;
    } else {
        b()?;
    }

    Ok(())
}

fn main() {
    env_logger::Builder::new()
        .format_timestamp(None)
        .filter_level(log::LevelFilter::Info)
        .init();

    eprintln!("Calling c(true):");

    let _ = c(true);

    eprintln!("Calling c(false):");

    let _ = c(false);
}
