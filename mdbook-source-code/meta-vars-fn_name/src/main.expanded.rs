#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use hooq::hooq;
fn failable<T>(val: T) -> Result<T, String> {
    Ok(val)
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    failable(())
        .inspect_err(|_| {
            let _fn_name = "main";
        })?;
    (|| -> Result<(), String> {
        failable(())
            .inspect_err(|_| {
                let _fn_name = "__closure_in_main__";
            })?;
        Ok(())
    })()
        .inspect_err(|_| {
            let _fn_name = "main";
        })?;
    Ok(())
}
