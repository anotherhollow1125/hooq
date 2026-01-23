#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use hooq::hooq;
fn fallible<T>(val: T) -> Result<T, String> {
    Ok(val)
}
fn main() -> Result<(), String> {
    let _: Result<(), String> = {
        let _: Result<(), String> = {
            let res = "error".to_string();
            Err(res).inspect_err(|_| {})
        };
        fallible(())
    };
    let _: Result<(), String> = {
        let _: Result<(), String> = {
            let res = "error".to_string();
            Err(res).inspect_err(|_| {})
        };
        fallible(()).inspect_err(|_| {})
    };
    Ok(())
}
