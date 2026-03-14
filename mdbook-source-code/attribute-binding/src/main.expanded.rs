#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
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
