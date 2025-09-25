use hooq_macros::hooq;

#[hooq]
fn enresult<T>(v: T) -> Result<T, ()> {
    Ok(v)
}

#[hooq]
#[hooq::method(.inspect(|_| {
    println!("tag: {}, expr: {}", $tag, $expr_str);
}))]
#[hooq::tag = "(no tag)"]
fn func() -> Result<(), ()> {
    let _ = #[hooq::tag = "outer"]
    vec![enresult(10)?; enresult(2)?];

    let _ = vec![
        #[hooq::tag = "inner 1"]
        enresult(10)?,
        #[hooq::tag = "inner 2"]
        enresult(20)?,
        #[hooq::tag = "inner 3"]
        enresult(30)?,
    ];

    Err(())
}

#[test]
fn test() {
    func().unwrap_err();
}
