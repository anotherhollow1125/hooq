#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use hooq::hooq;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    failable(())
        .inspect_err(|_| {
            let _ = "specified @ root";
        })?;
    failable(())
        .inspect_err(|_| {
            let _ = "specified @ inner";
        })?;
    Ok(())
}
fn failable<T>(val: T) -> Result<T, String> {
    Ok(val)
}
