#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use hooq::hooq;
fn failable<T>(val: T) -> Result<T, String> {
    Ok(val)
}
fn target_question() -> Result<(), String> {
    failable(()).inspect_err(|_| {})?;
    if failable(false).inspect_err(|_| {})? {
        return Err("error".into());
    }
    if failable(true).inspect_err(|_| {})? { Ok(()) } else { Err("error".into()) }
}
fn target_return() -> Result<(), String> {
    failable(())?;
    if failable(false)? {
        return Err("error".into()).inspect_err(|_| {});
    }
    if failable(true)? { Ok(()) } else { Err("error".into()) }
}
fn target_tail_expr() -> Result<(), String> {
    failable(())?;
    if failable(false)? {
        return Err("error".into());
    }
    if failable(true)? { Ok(()) } else { Err("error".into()).inspect_err(|_| {}) }
        .inspect_err(|_| {})
}
fn main() {
    let _ = target_question();
    let _ = target_return();
    let _ = target_tail_expr();
}
