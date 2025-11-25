#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use hooq::hooq;
fn failable<T>(val: T) -> Result<T, String> {
    Ok(val)
}
fn main() -> Result<(), String> {
    let _: Result<(), String> = {
        let _: Result<(), String> = {
            let res = "error".to_string();
            Err(res).inspect_err(|_| {})
        };
        failable(())
    };
    let _: Result<(), String> = {
        let _: Result<(), String> = {
            let res = "error".to_string();
            Err(res).inspect_err(|_| {})
        };
        failable(()).inspect_err(|_| {})
    };
    Ok(())
}
