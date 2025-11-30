#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use hooq::hooq;
fn failable<T>(val: T) -> Result<T, String> {
    Ok(val)
}
fn process(flag: bool) -> Result<(), String> {
    if flag {
        return Err("An error occurred".into());
    }
    let _ = failable(42)?;
    Ok(())
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    process(false)?;
    Ok(())
}
