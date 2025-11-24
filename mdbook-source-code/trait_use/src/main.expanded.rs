#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use hooq::hooq;
mod sub {
    pub trait Inserted {
        fn inserted(self) -> Self;
    }
    impl<T, E> Inserted for Result<T, E> {
        fn inserted(self) -> Self {
            self
        }
    }
}
fn failable<T>(val: T) -> Result<T, String> {
    Ok(val)
}
#[allow(unused)]
use sub::Inserted as _;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    failable(()).inserted()?;
    Ok(())
}
