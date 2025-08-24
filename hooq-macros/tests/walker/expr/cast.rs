use hooq_macros::hooq;

#[hooq]
fn hoge() -> Result<u32, ()> {
    Ok(10)
}

#[hooq]
#[hooq::method(.inspect(|_| {
    println!("tag: {}", $tag);
}))]
fn func() -> Result<(), ()> {
    let _ = hoge()? as i64;

    Ok(())
}

#[test]
fn test() {
    func().unwrap();
}
