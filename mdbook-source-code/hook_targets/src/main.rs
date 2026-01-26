use hooq::hooq;

fn fallible<T>(val: T) -> Result<T, String> {
    Ok(val)
}

#[hooq]
#[hooq::method(.inspect_err(|_| {}))]
#[hooq::hook_targets("?")]
fn target_question() -> Result<(), String> {
    fallible(())?;

    if fallible(false)? {
        return Err("error".into());
    }

    if fallible(true)? {
        Ok(())
    } else {
        Err("error".into())
    }
}

#[hooq]
#[hooq::method(.inspect_err(|_| {}))]
#[hooq::hook_targets("return")]
fn target_return() -> Result<(), String> {
    fallible(())?;

    if fallible(false)? {
        return Err("error".into());
    }

    if fallible(true)? {
        Ok(())
    } else {
        Err("error".into())
    }
}

#[hooq]
#[hooq::method(.inspect_err(|_| {}))]
#[hooq::hook_targets("tail_expr")]
fn target_tail_expr() -> Result<(), String> {
    fallible(())?;

    if fallible(false)? {
        return Err("error".into());
    }

    if fallible(true)? {
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
