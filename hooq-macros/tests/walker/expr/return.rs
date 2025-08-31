use hooq_macros::hooq;

#[hooq]
#[hooq::method(.inspect(|_| {
    println!("tag: {}", $tag);
}))]
#[hooq::tag = "(no tag)"]
fn func(flag: bool) -> Result<(), ()> {
    #[hooq::tag = "return"]
    if flag {
        return Ok(());
    }

    #[hooq::tag = "not result"]
    fn hoge(flag: bool) -> u32 {
        if flag {
            return 1;
        }
        0
    }

    let _ = hoge(flag);

    Ok(())
}

#[test]
fn test() {
    func(true).unwrap();
}
