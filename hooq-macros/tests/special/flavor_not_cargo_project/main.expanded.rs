#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
extern crate hooq;

use hooq::hooq;

fn hoge(flag: bool) -> Result<(), Box<dyn std::error::Error>> {
    if flag {
        return Err("error".into())
            .inspect_err(|v| {
                {
                    ::std::io::_eprint(format_args!("Custom Error Handler: {0}\n", v));
                };
            })
            .inspect(|_| {
                ::std::io::_print(format_args!("Custom Success Handler\n"));
            });
    }
    Ok(())
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    hoge(false)
        .inspect_err(|v| {
            {
                ::std::io::_eprint(format_args!("Default Error Handler: {0}\n", v));
            };
        })
        .inspect(|_| {
            ::std::io::_print(format_args!("Default Success Handler\n"));
        })?;
    Ok(())
}
