use std::fmt::Debug;

use hooq::hooq;

#[hooq(custom::ok)]
fn enresult<T: Debug>(val: T) -> Result<T, ()> {
    Ok(val)
}

type MyResult = Result<(), ()>;

#[allow(non_snake_case)]
fn MyOk() -> MyResult {
    Ok(())
}

#[allow(non_snake_case)]
fn MyErr() -> MyResult {
    Err(())
}

#[hooq]
fn flavor_override() -> Result<(), ()> {
    enresult(())?;

    #[hooq::flavor = "custom.full"]
    let f = || -> MyResult {
        enresult(())?;

        println!("{}", enresult(10)?);

        if enresult(false)? {
            return MyErr();
        }

        let g = || -> MyResult { enresult(()) };
        let h = || -> Result<(), ()> { enresult(()) };

        g()?;
        #[hooq::flavor = empty]
        h()?;

        let _ = { MyErr() };

        MyOk()
    };

    println!("{:?}", f()?);

    enresult(())?;

    Ok(())
}

#[hooq]
fn method_override() -> Result<(), ()> {
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

    enresult(())?;

    Ok(())
}

#[hooq(empty)]
#[hooq::method = custom]
#[hooq::hook_targets("?", "tail_expr", "return")]
#[hooq::tail_expr_idents("Ok", "Err")]
fn hook_targets_override() -> Result<(), ()> {
    let f = || {
        if enresult(false)? {
            return Err::<(), ()>(());
        }

        enresult(())?;

        if enresult(true)? {
            return Ok(());
        }

        Err::<(), ()>(())
    };

    f()?;

    #[hooq::hook_targets = custom::full] // return will be no longer hooked
    let f = || {
        if enresult(false)? {
            return Err::<(), ()>(());
        }

        enresult(())?;

        if enresult(true)? {
            return Ok(());
        }

        Err::<(), ()>(())
    };

    f()?;

    Ok(())
}

#[hooq(empty)]
#[hooq::method = custom]
#[hooq::hook_targets("?", "tail_expr", "return")]
#[hooq::tail_expr_idents("Ok", "Err")]
fn tail_expr_idents_override() -> Result<(), ()> {
    let f = || {
        enresult(())?;

        if enresult(false)? {
            return MyOk();
        }

        if enresult(false)? {
            return MyErr();
        }

        if enresult(false)? {
            return Err::<(), ()>(());
        }

        if enresult(true)? {
            return Ok(());
        }

        Ok(())
    };

    f()?;

    #[hooq::tail_expr_idents = custom::full] // MyErr will be hooked
    let f = || {
        enresult(())?;

        if enresult(false)? {
            return MyOk();
        }

        if enresult(false)? {
            return MyErr();
        }

        if enresult(false)? {
            return Err::<(), ()>(());
        }

        if enresult(true)? {
            return Ok(());
        }

        Ok(())
    };

    f()?;

    Ok(())
}

#[hooq(empty)]
#[hooq::method = custom]
#[hooq::hook_targets("?", "tail_expr", "return")]
#[hooq::tail_expr_idents("!Ok", "Err")]
#[hooq::result_types("Result")]
fn result_types_override() -> Result<(), ()> {
    let f = || -> Result<(), ()> {
        if enresult(false)? {
            return enresult(());
        }

        Ok(())
    };

    f()?;

    #[hooq::result_types = custom::full] // result_types will be overridden to `MyResult`.
    let f = || -> Result<(), ()> {
        if enresult(false)? {
            return enresult(());
        }

        Ok(())
    };

    f()?;

    #[hooq::result_types = custom::full] // result_types will be overridden to `MyResult`.
    let f = || -> MyResult {
        if enresult(false)? {
            return enresult(());
        }

        Ok(())
    };

    f()?;

    Ok(())
}

#[hooq]
fn hook_in_macros_override() -> Result<(), ()> {
    println!("{}", enresult(10)?);

    #[hooq::hook_in_macros = custom::full] // hook_in_macros will be overridden to `false`.
    println!("{}", enresult(10)?);

    Ok(())
}

#[hooq]
#[hooq::hello = "(not specified)"]
#[hooq::method(.inspect_err(|_| println!("{}", $hello)))]
fn bindings_override() -> Result<(), ()> {
    enresult(())?;

    #[hooq::bindings = custom::full]
    enresult(())?;

    Ok(())
}

#[test]
fn test() {
    flavor_override().unwrap();
    method_override().unwrap();
    hook_targets_override().unwrap();
    tail_expr_idents_override().unwrap();
    result_types_override().unwrap();
    hook_in_macros_override().unwrap();
    bindings_override().unwrap();
}
