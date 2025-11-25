#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use hooq::hooq;
fn failable<T>(val: T) -> Result<T, String> {
    Ok(val)
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    failable(())
        .inspect_err(|_| {
            let _line = 28usize;
            let _column = 17usize;
            let _path = "mdbook-source-code/meta-vars-all/src/main.rs";
            let _file = "main.rs";
            let _source = "failable(()) ?";
            let _count = "1st ?";
            let _fn_name = "main";
            let _fn_sig = "fn main() -> Result < (), Box < dyn std :: error :: Error > >";
            let _xxx = "user defined binding.";
            let _bindings = ::std::collections::HashMap::from([
                (
                    ::std::string::ToString::to_string("xxx"),
                    {
                        let expr = ::std::string::ToString::to_string(
                            "\"user defined binding.\"",
                        );
                        let value: ::std::rc::Rc<dyn ::std::any::Any> = ::std::rc::Rc::new(
                            "user defined binding.",
                        );
                        ::hooq::BindingPayload {
                            expr,
                            value,
                        }
                    },
                ),
            ]);
            let _hooq_meta = ::hooq::HooqMeta {
                line: 28usize,
                column: 17usize,
                path: "mdbook-source-code/meta-vars-all/src/main.rs",
                file: "main.rs",
                source_str: "failable(()) ?",
                count: "1st ?",
                bindings: ::std::collections::HashMap::from([
                    (
                        ::std::string::ToString::to_string("xxx"),
                        {
                            let expr = ::std::string::ToString::to_string(
                                "\"user defined binding.\"",
                            );
                            let value: ::std::rc::Rc<dyn ::std::any::Any> = ::std::rc::Rc::new(
                                "user defined binding.",
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
