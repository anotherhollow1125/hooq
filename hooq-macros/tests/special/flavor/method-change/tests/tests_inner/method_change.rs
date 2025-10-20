use std::fmt::Debug;

use hooq::hooq;

#[hooq(custom::ok)]
fn enresult<T: Debug>(val: T) -> Result<T, ()> {
    Ok(val)
}

#[hooq]
fn func() -> Result<(), ()> {
    enresult(())?;

    #[hooq::method = "custom.ok"]
    #[hooq::tail_expr_idents("Ok")]
    if enresult(false)? {
        return Ok(());
    }

    // method での上書きはメソッドしか上書きしない
    #[hooq::method = custom::ok]
    if enresult(false)? {
        return Ok(());
    }

    #[hooq::method = empty]
    enresult(())?;

    Ok(())
}

#[test]
fn test() {
    func().unwrap();
}
