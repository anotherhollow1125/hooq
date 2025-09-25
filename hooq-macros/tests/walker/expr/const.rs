use hooq_macros::hooq;

#[hooq]
#[allow(unused)]
fn hoge() -> Result<(), ()> {
    Err(())
}

#[hooq]
#[hooq::method(.inspect(|_| {
    println!("tag: {}", $tag);
}))]
#[hooq::tag = "(no tag)"]
fn func() -> Result<(), ()> {
    const _N: usize = const {
        #[hooq::tag = "in const block"]
        fn _f() -> Result<(), ()> {
            hoge()?;
            Err(())
        }
        10
    };

    Err(())
}

#[test]
fn test() {
    func().unwrap_err();
}
