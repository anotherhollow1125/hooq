use hooq::hooq;

#[hooq]
#[hooq::method(.inspect(|_| {
    println!("tag: {}", $tag);
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
