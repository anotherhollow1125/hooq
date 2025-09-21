use hooq_macros::hooq;

#[hooq]
fn hoge() -> Result<(), ()> {
    Ok(())
}

#[hooq]
fn default(flag: bool) -> Result<(), ()> {
    hoge()?;

    if !flag {
        return Err(());
    }

    Err(())
}

#[hooq]
#[hooq::hook_targets("?")]
fn q_only(flag: bool) -> Result<(), ()> {
    hoge()?;

    if !flag {
        return Err(());
    }

    Err(())
}

#[hooq]
#[hooq::hook_targets("return")]
fn return_only(flag: bool) -> Result<(), ()> {
    hoge()?;

    if !flag {
        return Err(());
    }

    Err(())
}

#[hooq]
#[hooq::hook_targets("tail_expr")]
fn tailexpr_only(flag: bool) -> Result<(), ()> {
    hoge()?;

    if !flag {
        return Err(());
    }

    Err(())
}

#[hooq]
#[hooq::hook_targets("return", "question")]
fn return_and_question(flag: bool) -> Result<(), ()> {
    hoge()?;

    if !flag {
        return Err(());
    }

    Err(())
}

#[test]
fn test() {
    default(true).unwrap_err();
    q_only(true).unwrap_err();
    return_only(true).unwrap_err();
    tailexpr_only(true).unwrap_err();
    return_and_question(true).unwrap_err();
}
