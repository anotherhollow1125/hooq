use hooq::hooq;
use tracing::instrument;

#[hooq(tracing)] // default event level is Error
#[instrument]
fn a() -> Result<(), String> {
    Err("An error occurred in function a".to_string())
}

#[hooq(tracing::warn)]
#[instrument]
fn b() -> Result<(), String> {
    a()?;

    Ok(())
}

#[hooq(tracing::info)]
#[instrument]
fn c(flag: bool) -> Result<(), String> {
    if flag {
        #[hooq::level = tracing::Level::ERROR]
        b()?;
    } else {
        b()?;
    }

    Ok(())
}

fn main() {
    let format = tracing_subscriber::fmt::format().without_time();
    tracing_subscriber::fmt()
        .with_ansi(false)
        .event_format(format)
        .init();

    println!("Calling c(true):");

    let _ = c(true);

    println!("Calling c(false):");

    let _ = c(false);
}
