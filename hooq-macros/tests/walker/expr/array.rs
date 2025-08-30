use hooq_macros::hooq;

#[hooq]
fn hoge() -> Result<u32, ()> {
    Ok(10)
}

#[hooq]
#[hooq::method(.inspect(|_| {
    println!("tag: {}", $tag);
}))]
#[hooq::tag = "(no tag)"]
fn func() -> Result<[u32; 2], ()> {
    Ok([
        #[hooq::tag = "first"]
        hoge()?,
        #[hooq::tag = "second"]
        hoge()?,
    ])
}

#[test]
fn test() {
    func().unwrap();
}
