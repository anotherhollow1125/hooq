#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use hooq::hooq;
mod hoge {
    pub fn bar() -> i32 {
        42
    }
    pub fn fallible<T>(val: T) -> Result<T, ()> {
        Ok(val)
    }
    pub fn _fallible_2<T>(val: T) -> Result<T, ()> {
        if fallible(false).unwrap() {
            return fallible(val).inspect_err(|_| {});
        }
        Ok(val).inspect_err(|_| {})
    }
    pub fn fuga() -> Result<(), ()> {
        fallible(()).inspect_err(|_| {})?;
        let _ = || {
            fallible(()).inspect_err(|_| {})?;
            if fallible(false).inspect_err(|_| {})? {
                return Err(()).inspect_err(|_| {});
            }
            Ok(())
        };
        let _ = || { Result::<(), ()>::Ok(()) };
        let _ = {
            let _ = bar();
            Result::<(), ()>::Err(()).inspect_err(|_| {})
        };
        let _ = || -> Result<(), ()> { fallible(()).inspect_err(|_| {}) };
        let _ = || { fallible(()) };
        Ok(())
    }
}
fn main() {
    let _ = hoge::bar();
    let _ = hoge::fallible(123);
    let _ = hoge::fuga();
}
