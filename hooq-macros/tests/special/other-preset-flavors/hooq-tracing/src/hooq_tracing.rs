use hooq::hooq;
use tracing::instrument;

#[hooq(tracing)]
#[instrument]
pub fn a() -> Result<(), String> {
    Err("error!".to_string())
}

#[hooq(tracing::error)]
#[instrument]
pub fn b() -> Result<(), String> {
    a()?;

    Err("error!".to_string())
}

#[hooq(tracing::warn)]
#[instrument]
pub fn c() -> Result<(), String> {
    b()?;

    Err("error!".to_string())
}

#[hooq(tracing::info)]
#[instrument]
pub fn d() -> Result<(), String> {
    c()?;

    Err("error!".to_string())
}

#[hooq(tracing::debug)]
#[instrument]
pub fn e() -> Result<(), String> {
    d()?;

    Err("error!".to_string())
}

#[hooq(tracing::trace)]
#[instrument]
pub fn f() -> Result<(), String> {
    e()?;

    Err("error!".to_string())
}
