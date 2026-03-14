#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
use hooq::hooq;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    fallible(()).inspect_err(|_| {})?;
    let f = || -> Option<()> {
        optional(())?;
        Some(())
    };
    let _ = fallible(f()).inspect_err(|_| {})?;
    Ok(())
}
fn fallible<T>(val: T) -> Result<T, String> {
    Ok(val)
}
fn optional<T>(val: T) -> Option<T> {
    Some(val)
}
