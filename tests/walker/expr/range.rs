use hooq::hooq;

#[hooq]
fn index(i: usize) -> Result<usize, ()> {
    Ok(i)
}

#[hooq]
#[hooq::method(.inspect(|_| {
    println!("tag: {}", $tag);
}))]
fn func() -> Result<(), ()> {
    let _ = index(0)?..index(10)?;
    let _ = index(0)?..=index(10)?;
    let _ = index(0)?..;
    let _ = ..index(10)?;
    let _ = ..=index(10)?;

    Ok(())
}

#[test]
fn test() {
    func().unwrap();
}
