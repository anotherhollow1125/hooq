#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use hooq::hooq;
fn failable<T>(val: T) -> Result<T, String> {
    Ok(val)
}
fn main() {
    let f = || -> Result<(), String> { failable(()).inspect_err(|_| {}) };
    let g = || -> Result<(), String> { failable(()) };
    let h = || -> Result<(), String> { failable(()) };
    f().unwrap();
    g().unwrap();
    h().unwrap();
}
