#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use hooq::hooq;
fn fallible<T>(val: T) -> Result<T, String> {
    Ok(val)
}
fn process(flag: bool) -> Result<(), String> {
    if flag {
        return Err("An error occurred".into());
    }
    let _ = fallible(42).unwrap();
    Ok(())
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    process(false).unwrap();
    Ok(())
}
