#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
use hooq::hooq;
fn fallible<T>(val: T) -> Result<T, String> {
    Ok(val)
}
fn wrapper<T, E>(r: Result<T, E>) -> Result<T, E>
where
    E: std::fmt::Debug,
{
    if let Err(e) = &r {
        {
            ::std::io::_print(format_args!("Error occurred: {0:?}\n", e));
        };
    }
    r
}
fn main() -> Result<(), String> {
    wrapper(fallible(()))?;
    Ok(())
}
