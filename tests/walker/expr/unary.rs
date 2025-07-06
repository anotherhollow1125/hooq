use hooq::hooq;

#[hooq]
fn enresult<T>(v: T) -> Result<T, ()> {
    Ok(v)
}

#[hooq]
#[hooq::method(.inspect(|_| {
    println!("tag: {}", $tag);
}))]
fn func() -> Result<(), ()> {
    let b = !enresult(true)?;
    let v = -enresult({
        if b {
            return Err(());
        }

        1
    })?;
    let v = *enresult(&v)? + 1;
    let _ = !enresult({
        if v > 0 {
            return enresult(());
        }

        #[hooq::tag("nested")]
        enresult(false)?
    })?;

    Ok(())
}

#[test]
fn test() {
    func().unwrap();
}
