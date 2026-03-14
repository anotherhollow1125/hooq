#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
use hooq::hooq;
fn fallible<T>(val: T) -> Result<T, String> {
    Ok(val)
}
#[allow(clippy::question_mark)]
fn main() -> Result<(), String> {
    let _ = match fallible(42) {
        Ok(val) => val,
        Err(err) => return Err(From::from(err)),
    };
    Ok(())
}
