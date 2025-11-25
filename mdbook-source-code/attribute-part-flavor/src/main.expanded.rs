#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use hooq::hooq;
fn failable<T>(val: T) -> Result<T, String> {
    Ok(val)
}
type MyResult = Result<(), String>;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    failable(())
        .inspect_err(|_| {
            let _x = "from root";
            let _y = "from root";
        })?;
    let _ = || -> MyResult { failable(()) };
    failable(())
        .inspect_err(|_| {
            let _ = "from my_flavor";
            let _x = "from root";
            let _y = "from root";
        })?;
    let _ = || -> MyResult {
        failable(())
            .inspect_err(|_| {
                let _x = "from root";
                let _y = "from root";
            })
    };
    failable(())
        .inspect_err(|_| {
            let _x = "xxx from my_flavor";
            let _y = "yyy from my_flavor";
        })?;
    failable(())
        .inspect_err(|_| {
            let _ = "from my_flavor";
            let _x = "xxx from my_flavor";
            let _y = "yyy from my_flavor";
        })?;
    Ok(())
}
