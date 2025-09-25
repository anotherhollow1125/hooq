use std::fmt::Debug;

use hooq::{hooq, toml_load};

toml_load!(
    r#"
[default]
method = """.inspect(|v| println!("Ok Value with: {v:?} & with tag: {}", $tag))
.inspect_err(|e| eprintln!("Err Value with: {e:?} & with tag: {}", $tag))"""
bindings = { tag = "\"[default]\"" }
"#
);

#[hooq]
#[allow(unused)]
fn enresult<T: Debug>(val: T) -> Result<T, ()> {
    Ok(val)
}

#[hooq]
#[allow(unused)]
fn func(flag: bool) -> Result<(), ()> {
    enresult(())?;

    if flag {
        return Err(());
    }

    Err(())
}

#[test]
fn test() {
    func(false).unwrap_err();
}
