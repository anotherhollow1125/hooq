#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use hooq::hooq;
fn failable<T>(val: T) -> Result<T, String> {
    Ok(val)
}
mod sub {
    pub trait MyTrait {}
}
#[allow(unused)]
use sub::MyTrait as _;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    failable(())
        .inspect_err(|_| {
            let _ = "my_flavor";
        })?;
    failable(())
        .inspect_err(|_| {
            let _ = "my_flavor_sub";
        })?;
    Ok(())
}
