use hooq::hooq;

#[hooq]
#[hooq::method(.map(|v| {
    println!("tag: {:?}", $tag);
    v
}))]
fn func() -> Result<(), ()> {
    struct S;

    #[hooq::tag("impl")]
    impl S {
        #[hooq::tag("impl related function")]
        fn g() -> Result<(), ()> {
            Ok(())
        }

        #[hooq::tag("impl related function 2 (not Result)")]
        fn h() -> bool {
            true
        }

        #[hooq::tag("impl method")]
        fn i(&self) -> Result<(), ()> {
            let res = Ok(());

            res
        }

        #[hooq::tag("impl method 2 (not Result)")]
        fn j(&self) -> bool {
            true
        }
    }

    Ok(())
}

fn main() {
    func().unwrap();
}
