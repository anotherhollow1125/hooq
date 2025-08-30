use hooq_macros::hooq;

#[hooq]
fn hoge() -> Result<(), ()> {
    Ok(())
}

#[hooq]
#[hooq::method(.inspect(|_| {
    println!("tag: {}", $tag);
}))]
#[hooq::tag = "(no tag)"]
fn func(flag: bool) -> Result<(), ()> {
    let _: ((), usize, Result<(), ()>) = (
        hoge()?,
        {
            if !flag {
                return Err(());
            }

            10
        },
        {
            if flag {
                return hoge();
            }

            Ok(())
        },
    );

    Ok(())
}

#[test]
fn test() {
    func(true).unwrap();
}
