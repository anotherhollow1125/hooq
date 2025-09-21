use std::fmt::Debug;

use hooq::{hooq, toml_load};

toml_load!(
    r#"
[my_flavor]
not_tail_expr_idents = []
method = """.inspect(|v| println!("Ok Value with: {v:?} & with tag: {}", $tag))
.inspect_err(|e| eprintln!("Err Value with: {e:?} & with tag: {}", $tag))"""
bindings = { tag = "\"[default]\"" }

[my_flavor.sub]
bindings = { tag = "\"[sub]\"" }

[my_flavor.sub.sub]
bindings = { tag = "\"[sub.sub]\"" }
tail_expr_idents = ["Ok", "Err"]

[my_flavor.not_tail_expr_idents_test_1]
bindings = { tag = "\"[not_tail_expr_idents_test_1]\"" }
tail_expr_idents = ["Err"]
not_tail_expr_idents = ["Ok"]

[my_flavor.sub.sub.not_tail_expr_idents_test_2]
bindings = { tag = "\"[not_tail_expr_idents_test_2]\"" }
tail_expr_idents = ["Err", "!Ok"]
"#
);

#[hooq(my_flavor)]
#[allow(unused)]
fn enresult<T: Debug>(val: T) -> Result<T, ()> {
    Ok(val)
}

#[hooq(my_flavor::sub)]
#[allow(unused)]
fn func(flag: bool) -> Result<(), ()> {
    enresult(())?;

    if flag {
        return Ok(());
    }

    let _ = { Result::<(), ()>::Ok(()) };

    Err(())
}

#[hooq(my_flavor::sub::sub)]
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

#[hooq(my_flavor::not_tail_expr_idents_test_1)]
#[allow(unused)]
fn func3(flag: bool) -> Result<(), ()> {
    enresult(())?;

    if flag {
        return Ok(());
    }

    let res = { Ok(()) };

    println!("res: {res:?}");

    res
}

#[hooq(my_flavor::sub::sub::not_tail_expr_idents_test_2)]
#[allow(unused)]
fn func4(flag: bool) -> Result<(), ()> {
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
