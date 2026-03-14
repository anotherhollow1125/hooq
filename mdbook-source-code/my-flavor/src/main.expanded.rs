#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
use hooq::hooq;
fn fallible<T>(val: T) -> Result<T, String> {
    Ok(val)
}
mod sub {
    pub trait MyTrait {}
}
#[allow(unused)]
use sub::MyTrait as _;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    fallible(())
        .inspect_err(|_| {
            let _ = "my_flavor";
        })?;
    fallible(())
        .inspect_err(|_| {
            let _ = "my_flavor_sub";
        })?;
    Ok(())
}
