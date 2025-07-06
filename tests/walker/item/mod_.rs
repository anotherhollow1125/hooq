use hooq::hooq;

#[hooq]
#[hooq::method(.inspect(|_| {
    println!("tag: {}", $tag);
}))]
fn func() -> Result<(), ()> {
    #[hooq::tag("mod")]
    mod tmp {
        #[allow(unused)]
        fn func() -> Result<(), ()> {
            Ok(())
        }
    }

    Ok(())
}

#[test]
fn test() {
    func().unwrap();
}
