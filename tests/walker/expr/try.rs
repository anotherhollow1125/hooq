use hooq::hooq;

#[hooq]
#[hooq::method(.inspect(|_| {
    println!("tag: {:?}", $tag);
}))]
fn func() -> Result<(), ()> {
    #[hooq::tag("try")]
    #[allow(clippy::redundant_closure_call)]
    (#[hooq::tag("inner")]
    || {
        (#[hooq::tag("inner inner")]
        // このクロージャのOk(())にフックが掛からないのは仕様
        || Ok(()))()?;
        Ok(())
    })()?;

    Ok(())
}

#[test]
fn test() {
    func().unwrap();
}
