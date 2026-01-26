#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use hooq::hooq;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    func()
        .inspect(|_| {
            let _ = "my_flavor";
        })?;
    Ok(())
}
fn fallible<T>(val: T) -> Result<T, String> {
    Ok(val)
        .inspect(|_| {
            let _ = "my_flavor.sub_flavor";
        })
}
fn func() -> Result<(), Box<dyn std::error::Error>> {
    fallible(())
        .inspect(|_| {
            let _ = "my_flavor.sub_flavor";
        })?;
    Ok(())
        .inspect(|_| {
            let _ = "my_flavor.sub_flavor";
        })
}
