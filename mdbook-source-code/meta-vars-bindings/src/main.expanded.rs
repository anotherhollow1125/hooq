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
    #[allow(unused)]
    DataBase,
    Server,
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    failable(())
        .inspect_err(|_| {
            let _bindings = ::std::collections::HashMap::from([
                (
                    ::std::string::ToString::to_string("cause_kind"),
                    {
                        let expr = ::std::string::ToString::to_string(
                            "CauseKind :: Server",
                        );
                        let value: ::std::rc::Rc<dyn ::std::any::Any> = ::std::rc::Rc::new(
                            CauseKind::Server,
                        );
                        ::hooq::BindingPayload {
                            expr,
                            value,
                        }
                    },
                ),
                (
                    ::std::string::ToString::to_string("integer"),
                    {
                        let expr = ::std::string::ToString::to_string("10");
                        let value: ::std::rc::Rc<dyn ::std::any::Any> = ::std::rc::Rc::new(
                            10,
                        );
                        ::hooq::BindingPayload {
                            expr,
                            value,
                        }
                    },
                ),
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
            ]);
        })?;
    Ok(())
}
