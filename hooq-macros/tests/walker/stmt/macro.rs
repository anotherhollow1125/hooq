use hooq_macros::hooq;

#[hooq]
fn enresult<T>(v: T) -> Result<T, ()> {
    Ok(v)
}

#[hooq]
#[hooq::method(.inspect(|_| {
    println!("tag: {}, expr: {}", $tag, $expr);
}))]
fn func() -> Result<(), ()> {
    #[hooq::tag("outer")]
    println!("{}", enresult(10)?);

    println!(
        "{}",
        #[hooq::tag("inner")]
        enresult(20)?
    );

    Ok(())
}

#[test]
fn test() {
    func().unwrap();
}
