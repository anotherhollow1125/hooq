#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use hooq::hooq;
fn fallible<T>(val: T) -> Result<T, String> {
    Ok(val)
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    fallible(())
        .inspect_err(|_| {
            let _ = 10;
        })?;
    {
        fallible(())
            .inspect_err(|_| {
                let _ = "in block";
            })?;
        fallible(())
            .inspect_err(|_| {
                let _ = 42;
            })?;
        fallible(())
            .inspect_err(|_| {
                let _ = "in block";
            })?;
    }
    fallible(())
        .inspect_err(|_| {
            let _ = 10;
        })?;
    Ok(())
}
