#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use hooq::hooq;
fn failable<T>(val: T) -> Result<T, String> {
    Ok(val)
}
type MyResult = Result<(), String>;
fn func1() -> MyResult {
    let _ = || -> Result<(), String> { failable(()).inspect_err(|_| {}) };
    failable(())
}
fn func2() -> MyResult {
    let _ = || -> Result<(), String> { failable(()) };
    failable(()).inspect_err(|_| {})
}
fn func3() -> MyResult {
    let _ = || -> Result<(), String> { failable(()).inspect_err(|_| {}) };
    let _ = || { failable(()) };
    failable(()).inspect_err(|_| {})
}
fn main() {
    let _ = func1();
    let _ = func2();
    let _ = func3();
}
