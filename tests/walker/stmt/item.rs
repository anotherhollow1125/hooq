use hooq::hooq;

#[hooq]
#[hooq::method(.map(|v| {
    println!("tag: {:?}", $tag);
    v
}))]
fn func() -> Result<(), ()> {
    #[hooq::tag("func")]
    fn f() -> Result<(), ()> {
        Ok(())
    }

    f()?;

    Ok(())
}

#[test]
fn test() {
    func().unwrap();
}
