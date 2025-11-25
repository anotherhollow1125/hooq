#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use hooq::hooq;
fn failable<T>(val: T) -> Result<T, String> {
    Ok(val)
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    failable(())
        .inspect_err(|_| {
            let _ = 10;
        })?;
    {
        failable(())
            .inspect_err(|_| {
                let _ = "in block";
            })?;
        failable(())
            .inspect_err(|_| {
                let _ = 42;
            })?;
        failable(())
            .inspect_err(|_| {
                let _ = "in block";
            })?;
    }
    failable(())
        .inspect_err(|_| {
            let _ = 10;
        })?;
    Ok(())
}
