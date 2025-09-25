use hooq_macros::hooq;

#[hooq]
fn hoge(v: usize) -> Result<usize, ()> {
    Ok(v * 2)
}

#[hooq]
#[hooq::method(.inspect(|_| {
    println!("{}", $tag);
}))]
#[hooq::tag = "(no tag)"]
fn func() -> Result<(), ()> {
    let mut x;

    x = hoge(1)?;

    let _ = x;

    #[hooq::tag = "outer"]
    #[allow(clippy::let_unit_value)]
    let _ = {
        x = #[hooq::tag = "inner"]
        hoge(2)?
    };

    let _ = x;

    Err(())
}

#[test]
fn test() {
    func().unwrap_err();
}
