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
            let _ = "inserted mode";
        })?;
    failable(())
        .inspect_err(|_| {
            let _ = "before chainned";
        })
        .inspect_err(|_| {
            let _ = "inserted mode";
        })?;
    failable(())
        .inspect_err(|_| {
            let _ = "inserted mode";
        })
        .inspect_err(|_| {
            let _ = "after chainned";
        })?;
    Ok(())
}
