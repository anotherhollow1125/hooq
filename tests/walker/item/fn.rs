use hooq::hooq;

#[hooq]
#[hooq::method(.map(|v| {
    println!("tag: {:?}", $tag);
    v
}))]
fn func() -> Result<(), ()> {
    #[hooq::tag("func")]
    fn f() -> Result<(), ()> {
        #[hooq::tag("func inner 1")]
        fn g() -> Result<(), ()> {
            Err(())
        }

        #[hooq::tag("func inner 2 (not Result)")]
        fn h() -> bool {
            println!("Hello, world!");
            true
        }

        let _ = g();
        let _ = h();

        Ok(())
    }

    f()?;

    Ok(())
}

#[test]
fn test() {
    func().unwrap();
}
