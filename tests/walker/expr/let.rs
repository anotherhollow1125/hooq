use hooq::hooq;

#[hooq]
fn hoge(v: usize) -> Result<usize, ()> {
    Ok(v + 1)
}

#[hooq]
#[hooq::method(.inspect(|_| {
    println!("tag: {:?}", $tag);
}))]
fn func() -> Result<(), ()> {
    if let 11 = hoge(10)?
        && #[hooq::tag("second let")]
        let 12 = hoge(11)?
    {
        println!("hoge is 10");

        let _ = hoge(0)?;

        hoge(0).map(|_| ())
    } else {
        println!("hoge is not 10");

        let _ = hoge(0)?;

        Err(())
    }
}

#[test]
fn test() {
    func().unwrap();
}
