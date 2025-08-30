use hooq_macros::hooq;

#[hooq]
#[hooq::method(.inspect(|_| {
    println!("tag: {}", $tag);
}))]
#[hooq::tag = "(no tag)"]
fn func() -> Result<(), ()> {
    #[hooq::tag = "func"]
    fn f() -> Result<(), ()> {
        #[hooq::tag = "func inner 1"]
        fn g() -> Result<(), ()> {
            Err(())
        }

        #[hooq::tag = "func inner 2 (not Result)"]
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
