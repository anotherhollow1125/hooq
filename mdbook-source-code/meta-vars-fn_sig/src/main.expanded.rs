#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use hooq::hooq;
fn fallible<T>(val: T) -> Result<T, String> {
    Ok(val)
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    fallible(())
        .inspect_err(|_| {
            let _fn_sig = "fn main() -> Result < (), Box < dyn std :: error :: Error > >";
        })?;
    (|| -> Result<(), String> {
        fallible(())
            .inspect_err(|_| {
                let _fn_sig = "| | -> Result < (), String > {}";
            })?;
        Ok(())
    })()
        .inspect_err(|_| {
            let _fn_sig = "fn main() -> Result < (), Box < dyn std :: error :: Error > >";
        })?;
    Ok(())
}
