use hooq_macros::hooq;

#[hooq]
fn enresult<T>(v: T) -> Result<T, ()> {
    Ok(v)
}

#[hooq]
#[hooq::method(.inspect(|_| {
    println!("tag: {}, expr: {}", $tag, stringify!($source));
}))]
#[hooq::tag = "(no tag)"]
fn func() -> Result<(), ()> {
    #[hooq::tag = "outer"]
    println!("{}", enresult(10)?);

    println!(
        "{}",
        #[hooq::tag = "inner"]
        enresult(20)?
    );

    Err(())
}

#[test]
fn test() {
    func().unwrap_err();
}
