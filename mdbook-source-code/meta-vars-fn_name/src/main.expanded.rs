#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
use hooq::hooq;
fn fallible<T>(val: T) -> Result<T, String> {
    Ok(val)
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    fallible(())
        .inspect_err(|_| {
            let _fn_name = "main";
        })?;
    (|| -> Result<(), String> {
        fallible(())
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
