use std::fmt::Debug;
use hooq::{hooq, toml_load};
#[allow(unused)]
fn enresult<T: Debug>(val: T) -> Result<T, ()> {
    Ok(val)
        .inspect(|v| {
            ::std::io::_print(
                format_args!("Ok Value with: {1:?} & with tag: {0}\n", "[default]", v),
            );
        })
        .inspect_err(|e| {
            ::std::io::_eprint(
                format_args!("Err Value with: {1:?} & with tag: {0}\n", "[default]", e),
            );
        })
}
#[allow(unused)]
fn func(flag: bool) -> Result<(), ()> {
    enresult(())
        .inspect(|v| {
            ::std::io::_print(
                format_args!("Ok Value with: {1:?} & with tag: {0}\n", "[default]", v),
            );
        })
        .inspect_err(|e| {
            ::std::io::_eprint(
                format_args!("Err Value with: {1:?} & with tag: {0}\n", "[default]", e),
            );
        })?;
    if flag {
        return Ok(())
            .inspect(|v| {
                ::std::io::_print(
                    format_args!(
                        "Ok Value with: {1:?} & with tag: {0}\n", "[default]", v
                    ),
                );
            })
            .inspect_err(|e| {
                ::std::io::_eprint(
                    format_args!(
                        "Err Value with: {1:?} & with tag: {0}\n", "[default]", e
                    ),
                );
            });
    }
    Err(())
        .inspect(|v| {
            ::std::io::_print(
                format_args!("Ok Value with: {1:?} & with tag: {0}\n", "[default]", v),
            );
        })
        .inspect_err(|e| {
            ::std::io::_eprint(
                format_args!("Err Value with: {1:?} & with tag: {0}\n", "[default]", e),
            );
        })
}
