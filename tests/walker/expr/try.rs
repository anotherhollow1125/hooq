use hooq::hooq;

#[hooq]
#[hooq::method(.map(|v| {
    println!("tag: {:?}", $tag);
    v
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
