use hooq::hooq;

#[hooq]
fn hoge() -> Result<usize, ()> {
    Ok(10)
}

#[hooq]
#[hooq::method(.inspect(|_| {
    println!("{}", $tag);
}))]
fn func() -> Result<(), ()> {
    let x;

    #[hooq::tag("outer")]
    #[allow(clippy::let_unit_value)]
    let _ = {
        x = #[hooq::tag("inner")]
        hoge()?
    };

    dbg!(x);

    Ok(())
}

#[test]
fn test() {
    func().unwrap();
}
