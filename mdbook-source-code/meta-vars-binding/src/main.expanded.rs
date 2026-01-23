#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use hooq::hooq;
fn fallible<T>(val: T) -> Result<T, String> {
    Ok(val)
}
enum CauseKind {
    DataBase,
    Server,
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    fallible(())
        .inspect_err(|_| {
            let _string = "hello!";
            let _integer = 10;
            let _cause_kind = CauseKind::Server;
        })?;
    fallible(())
        .inspect_err(|_| {
            let _string = "hello!";
            let _integer = 10;
            let _cause_kind = CauseKind::DataBase;
        })?;
    Ok(())
}
