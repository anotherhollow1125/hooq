#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use hooq::hooq;
fn failable<T>(val: T) -> Result<T, String> {
    Ok(val)
}
enum CauseKind {
    DataBase,
    Server,
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    failable(())
        .inspect_err(|_| {
            let _string = "hello!";
            let _integer = 10;
            let _cause_kind = CauseKind::Server;
        })?;
    failable(())
        .inspect_err(|_| {
            let _string = "hello!";
            let _integer = 10;
            let _cause_kind = CauseKind::DataBase;
        })?;
    Ok(())
}
