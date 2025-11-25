#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use hooq::hooq;
mod my_error {
    pub trait MyHook {
        fn hook(self, meta_fn: impl FnOnce() -> hooq::HooqMeta) -> Self;
    }
    impl<T, E> MyHook for Result<T, E>
    where
        E: std::fmt::Debug,
    {
        fn hook(self, meta_fn: impl FnOnce() -> hooq::HooqMeta) -> Self {
            if let Err(e) = &self {
                let meta = meta_fn();
                {
                    ::std::io::_eprint(
                        format_args!(
                            "[{0}:{1}:{2}] error occurred: {3:?}\n",
                            meta.file,
                            meta.line,
                            meta.column,
                            e,
                        ),
                    );
                };
            }
            self
        }
    }
}
fn failable<T>(val: T) -> Result<T, String> {
    Ok(val)
}
#[allow(unused)]
use my_error::MyHook as _;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    failable(())
        .hook(|| {
            ::hooq::HooqMeta {
                line: 33usize,
                column: 17usize,
                path: "mdbook-source-code/flavor-hook/src/main.rs",
                file: "main.rs",
                source_str: "failable(()) ?",
                count: "1st ?",
                bindings: ::std::collections::HashMap::from([]),
            }
        })?;
    Ok(())
}
