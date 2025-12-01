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
    pub fn failable<T>(val: T) -> Result<T, ()> {
        Ok(val)
    }
    pub fn _failable_2<T>(val: T) -> Result<T, ()> {
        if failable(false).unwrap() {
            return failable(val).inspect_err(|_| {});
        }
        Ok(val).inspect_err(|_| {})
    }
    pub fn fuga() -> Result<(), ()> {
        failable(()).inspect_err(|_| {})?;
        let _ = || {
            failable(()).inspect_err(|_| {})?;
            if failable(false).inspect_err(|_| {})? {
                return Err(()).inspect_err(|_| {});
            }
            Ok(())
        };
        let _ = || { Result::<(), ()>::Ok(()) };
        let _ = {
            let _ = bar();
            Result::<(), ()>::Err(()).inspect_err(|_| {})
        };
        let _ = || -> Result<(), ()> { failable(()).inspect_err(|_| {}) };
        let _ = || { failable(()) };
        Ok(())
    }
}
fn main() {
    let _ = hoge::bar();
    let _ = hoge::failable(123);
    let _ = hoge::fuga();
}
