use hooq::hooq;

#[hooq]
#[hooq::method(.map(|v| {
    println!("tag: {:?}", $tag);
    v
}))]
fn func() -> Result<(), ()> {
    #[allow(unused)]
    struct S;

    #[hooq::tag("impl")]
    impl S {
        #[allow(unused)]
        #[hooq::tag("impl related function")]
        fn g() -> Result<(), ()> {
            Ok(())
        }

        #[hooq::tag("impl related function 2 (not Result)")]
        #[allow(unused)]
        fn h() -> bool {
            true
        }

        #[allow(unused)]
        #[hooq::tag("impl method")]
        fn i(&self) -> Result<(), ()> {
            let res = Ok(());

            res
        }

        #[hooq::tag("impl method 2 (not Result)")]
        #[allow(unused)]
        fn j(&self) -> bool {
            true
        }
    }

    Ok(())
}

#[test]
fn test() {
    func().unwrap();
}
