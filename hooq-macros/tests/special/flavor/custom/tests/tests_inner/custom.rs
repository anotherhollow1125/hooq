use std::fmt::Debug;

use hooq::hooq;

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

#[hooq(my_flavor::ignore_tail_expr_idents_test_1)]
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

#[hooq(my_flavor::sub::sub::ignore_tail_expr_idents_test_2)]
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
    func2(true).unwrap();
    func2(false).unwrap();
    func3(true).unwrap();
    func3(false).unwrap();
    func4(true).unwrap();
    func4(false).unwrap();
}
