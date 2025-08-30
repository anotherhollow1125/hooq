use hooq_macros::hooq;

#[hooq]
#[hooq::method(.inspect(|_| {
    println!("tag: {}", $tag);
}))]
#[hooq::tag = "(no tag)"]
fn func() -> Result<(), ()> {
    #[hooq::tag = "func"]
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
