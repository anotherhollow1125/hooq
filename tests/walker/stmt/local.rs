use hooq::hooq;

#[hooq]
#[hooq::method(.inspect(|_| {
    println!("tag: {:?}", $tag);
}))]
fn func() -> Result<(), ()> {
    #[allow(clippy::redundant_closure_call)]
    let _ = #[hooq::tag("outer")]
    (|| {
        #[hooq::tag("inner")]
        Ok(true)
    })()?;

    let n = 1;

    #[hooq::tag("else")]
    let 1 = n else {
        return Err(());
    };

    Ok(())
}

#[test]
fn test() {
    func().unwrap();
}
