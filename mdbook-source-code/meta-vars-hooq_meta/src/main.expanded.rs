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
            let _hooq_meta = ::hooq::HooqMeta {
                line: 13usize,
                column: 17usize,
                path: "mdbook-source-code/meta-vars-hooq_meta/src/main.rs",
                file: "main.rs",
                source_str: "fallible(()) ?",
                count: "1st ?",
                bindings: ::std::collections::HashMap::from([
                    (
                        ::std::string::ToString::to_string("string"),
                        {
                            let expr = ::std::string::ToString::to_string("\"hello!\"");
                            let value: ::std::rc::Rc<dyn ::std::any::Any> = ::std::rc::Rc::new(
                                "hello!",
                            );
                            ::hooq::BindingPayload {
                                expr,
                                value,
                            }
                        },
                    ),
                ]),
            };
        })?;
    Ok(())
}
