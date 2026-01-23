#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use hooq::hooq;
fn fallible<T>(val: T) -> Result<T, String> {
    Ok(val)
}
type MyResult = Result<(), String>;
fn func1() -> MyResult {
    let _ = || -> Result<(), String> { fallible(()).inspect_err(|_| {}) };
    fallible(())
}
fn func2() -> MyResult {
    let _ = || -> Result<(), String> { fallible(()) };
    fallible(()).inspect_err(|_| {})
}
fn func3() -> MyResult {
    let _ = || -> Result<(), String> { fallible(()).inspect_err(|_| {}) };
    let _ = || { fallible(()) };
    fallible(()).inspect_err(|_| {})
}
fn main() {
    let _ = func1();
    let _ = func2();
    let _ = func3();
}
