use hooq_macros::hooq;

#[hooq]
#[hooq::method(.inspect(|_| {
    println!("tag: {}", $tag);
}))]
#[hooq::tag = "(no tag)"]
fn func() -> Result<(), ()> {
    #[hooq::tag = "try"]
    #[allow(clippy::redundant_closure_call)]
    (#[hooq::tag = "inner"]
    || {
        (#[hooq::tag = "inner inner"]
        || Ok(()))()?;
        Ok(())
    })()?;

    Ok(())
}

#[test]
fn test() {
    func().unwrap();
}
