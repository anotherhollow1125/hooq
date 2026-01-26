#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use hooq::hooq;
fn fallible<T>(val: T) -> Result<T, &'static str> {
    Ok(val)
}
fn main() {
    fallible(()).unwrap();
}
