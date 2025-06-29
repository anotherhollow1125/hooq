use hooq::hooq;

#[hooq]
fn hoge(v: usize) -> Result<usize, ()> {
    Ok(v * 2)
}

#[hooq]
#[hooq::method(.inspect(|_| {
    println!("{}", $tag);
}))]
fn func() -> Result<(), ()> {
    let mut x;

    x = hoge(1)?;

    dbg!(x);

    #[hooq::tag("outer")]
    #[allow(clippy::let_unit_value)]
    let _ = {
        x = #[hooq::tag("inner")]
        hoge(2)?
    };

    dbg!(x);

    Ok(())
}

#[test]
fn test() {
    func().unwrap();
}
