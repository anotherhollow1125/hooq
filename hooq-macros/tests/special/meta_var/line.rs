use hooq_macros::hooq;

fn enresult<T>(t: T) -> Result<T, ()> {
    Ok(t)
}

#[hooq]
#[hooq::method(.inspect_err(|_| {
    println!("line!(): {} $line: {}", line!(), $line);
}))]
fn func() -> Result<(), ()> {
    enresult(())?;

    Err(())
}

#[test]
fn test() {
    let _ = func();
}
