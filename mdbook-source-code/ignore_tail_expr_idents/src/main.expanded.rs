#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
use hooq::hooq;
fn fallible<T>(val: T) -> Result<T, String> {
    Ok(val)
}
fn main() {
    let f = || -> Result<(), String> { fallible(()).inspect_err(|_| {}) };
    let g = || -> Result<(), String> { fallible(()) };
    let h = || -> Result<(), String> { fallible(()) };
    f().unwrap();
    g().unwrap();
    h().unwrap();
}
