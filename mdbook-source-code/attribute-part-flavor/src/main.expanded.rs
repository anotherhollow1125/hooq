#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use hooq::hooq;
fn fallible<T>(val: T) -> Result<T, String> {
    Ok(val)
}
type MyResult = Result<(), String>;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    fallible(())
        .inspect_err(|_| {
            let _x = "from root";
            let _y = "from root";
        })?;
    let _ = || -> MyResult { fallible(()) };
    fallible(())
        .inspect_err(|_| {
            let _ = "from my_flavor";
            let _x = "from root";
            let _y = "from root";
        })?;
    let _ = || -> MyResult {
        fallible(())
            .inspect_err(|_| {
                let _x = "from root";
                let _y = "from root";
            })
    };
    fallible(())
        .inspect_err(|_| {
            let _x = "xxx from my_flavor";
            let _y = "yyy from my_flavor";
        })?;
    fallible(())
        .inspect_err(|_| {
            let _ = "from my_flavor";
            let _x = "xxx from my_flavor";
            let _y = "yyy from my_flavor";
        })?;
    Ok(())
}
