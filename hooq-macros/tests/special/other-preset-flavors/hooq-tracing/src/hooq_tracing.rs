use hooq::hooq;
use tracing::instrument;

#[hooq(tracing)]
#[instrument]
pub fn a() -> Result<(), String> {
    Err("error!".to_string())
}

#[hooq(tracing)]
#[instrument]
pub fn b() -> Result<(), String> {
    a()?;

    Err("error!".to_string())
}

#[hooq(tracing)]
#[instrument]
pub fn c() -> Result<(), String> {
    b()?;

    Err("error!".to_string())
}
