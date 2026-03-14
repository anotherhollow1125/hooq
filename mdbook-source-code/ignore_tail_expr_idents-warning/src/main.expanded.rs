#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
use hooq::hooq;
fn main() {
    let f = || -> Result<(), String> { Err("error!".to_string()).inspect_err(|_| {}) };
    let g = || -> Result<(), String> { Err("error!".to_string()).inspect_err(|_| {}) };
    let h = || -> Result<(), String> { Err("error!".to_string()) };
    f().unwrap_err();
    g().unwrap_err();
    h().unwrap_err();
}
