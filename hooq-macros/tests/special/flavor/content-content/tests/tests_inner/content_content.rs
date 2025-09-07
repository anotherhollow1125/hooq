use std::fmt::Debug;

use hooq::{hooq, toml_load};

toml_load!(
    content = r#"
[content]
method = """.inspect(|v| println!("Ok Value with: {v:?} & with tag: {}", $tag))
.inspect_err(|e| eprintln!("Err Value with: {e:?} & with tag: {}", $tag))"""
bindings = { tag = "\"[default]\"" }

[content.sub]
bindings = { tag = "\"[sub]\"" }

[content.bus]
bindings = { tag = "\"[bus]\"" }

[content.sub.sub]
bindings = { tag = "\"[sub.sub]\"" }
tail_expr_idents = ["Ok", "Err"]
"#
);

#[hooq(content)]
#[allow(unused)]
fn enresult<T: Debug>(val: T) -> Result<T, ()> {
    Ok(val)
}

#[hooq(content::bus)]
#[allow(unused)]
fn func(flag: bool) -> Result<(), ()> {
    enresult(())?;

    if flag {
        return Ok(());
    }

    let _ = { Result::<(), ()>::Ok(()) };

    Err(())
}

#[hooq(flavor = "content.sub::sub")]
#[allow(unused)]
fn func2(flag: bool) -> Result<(), ()> {
    enresult(())?;

    if flag {
        return Ok(());
    }

    let res = { Ok(()) };

    println!("res: {res:?}");

    res
}

#[test]
fn test() {
    func(false).unwrap_err();
    func(true).unwrap();
}
