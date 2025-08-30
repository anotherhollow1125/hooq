use hooq_macros::hooq;

struct Hoge {
    field: u32,
}

#[hooq]
fn hoge() -> Result<Hoge, ()> {
    Ok(Hoge { field: 10 })
}

#[hooq]
#[hooq::method(.inspect(|_| {
    println!("tag: {}", $tag);
}))]
#[hooq::tag = "(no tag)"]
fn func() -> Result<(), ()> {
    let _ = hoge()?.field;

    let _ = #[hooq::tag = "outer"]
    {
        #[hooq::tag = "inner"]
        hoge()?
    }
    .field;

    Ok(())
}

#[test]
fn test() {
    func().unwrap();
}
