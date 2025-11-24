#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use hooq::hooq;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    failable(()).inspect_err(|_| {})?;
    let f = || -> Option<()> {
        optional(())?;
        Some(())
    };
    let _ = failable(f()).inspect_err(|_| {})?;
    Ok(())
}
fn failable<T>(val: T) -> Result<T, String> {
    Ok(val)
}
fn optional<T>(val: T) -> Option<T> {
    Some(val)
}
