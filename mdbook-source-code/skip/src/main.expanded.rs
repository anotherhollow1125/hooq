#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use hooq::hooq;
fn main() -> Result<(), String> {
    match failable(failable(failable(()))) {
        Ok(v) => {
            match v {
                Ok(v) => {
                    match v {
                        Ok(v) => Ok(v),
                        Err(s) => Err(s),
                    }
                }
                Err(s) => Err(s),
            }
        }
        Err(s) => Err(s),
    }
}
fn failable<T>(val: T) -> Result<T, String> {
    Ok(val)
}
