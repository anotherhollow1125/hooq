use hooq::hooq;

#[hooq]
#[allow(unused)]
fn hoge() -> Result<(), ()> {
    Ok(())
}

#[hooq]
#[hooq::method(.inspect(|_| {
    println!("tag: {}", $tag);
}))]
fn func() -> Result<(), ()> {
    const _N: usize = const {
        #[hooq::tag("in const block")]
        fn _f() -> Result<(), ()> {
            hoge()?;
            Ok(())
        }
        10
    };

    Ok(())
}

#[test]
fn test() {
    func().unwrap();
}
