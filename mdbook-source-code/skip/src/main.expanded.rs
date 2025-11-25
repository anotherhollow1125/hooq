#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use hooq::hooq;
fn func1() -> Result<(), String> {
    match failable(failable(())).inspect_err(|_| {})? {
        Ok(()) => Ok(()),
        Err(s) => Err(s).inspect_err(|_| {}),
    }
        .inspect_err(|_| {})
}
fn func2() -> Result<(), String> {
    match failable(failable(()))? {
        Ok(()) => Ok(()),
        Err(s) => Err(s).inspect_err(|_| {}),
    }
}
fn failable<T>(val: T) -> Result<T, String> {
    Ok(val)
}
fn main() {
    let _ = func1();
    let _ = func2();
}
