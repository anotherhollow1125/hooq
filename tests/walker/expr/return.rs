use hooq::hooq;

#[hooq]
#[hooq::method(.map(|v| {
    println!("tag: {:?}", $tag);
    v
}))]
fn func(flag: bool) -> Result<(), ()> {
    #[hooq::tag("return")]
    if flag {
        return Ok(());
    }

    #[hooq::tag("not result")]
    fn hoge(flag: bool) -> u32 {
        if flag {
            return 1;
        }
        0
    }

    Ok(())
}

#[test]
fn test() {
    func(true).unwrap();
}
