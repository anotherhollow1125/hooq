#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
use hooq::hooq;
fn fallible<T>(val: T) -> Result<T, &'static str> {
    Ok(val)
}
fn main() {
    fallible(()).unwrap();
}
