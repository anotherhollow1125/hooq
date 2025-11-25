use hooq::hooq;

fn failable<T>(val: T) -> Result<T, String> {
    Ok(val)
}

#[hooq]
#[hooq::method(.inspect_err(|_| {}))]
#[hooq::hook_targets("?")]
fn target_question() -> Result<(), String> {
    failable(())?;

    if failable(false)? {
        return Err("error".into());
    }

    if failable(true)? {
        Ok(())
    } else {
        Err("error".into())
    }
}

#[hooq]
#[hooq::method(.inspect_err(|_| {}))]
#[hooq::hook_targets("return")]
fn target_return() -> Result<(), String> {
    failable(())?;

    if failable(false)? {
        return Err("error".into());
    }

    if failable(true)? {
        Ok(())
    } else {
        Err("error".into())
    }
}

#[hooq]
#[hooq::method(.inspect_err(|_| {}))]
#[hooq::hook_targets("tail_expr")]
fn target_tail_expr() -> Result<(), String> {
    failable(())?;

    if failable(false)? {
        return Err("error".into());
    }

    if failable(true)? {
        Ok(())
    } else {
        Err("error".into())
    }
}

fn main() {
    let _ = target_question();
    let _ = target_return();
    let _ = target_tail_expr();
}
