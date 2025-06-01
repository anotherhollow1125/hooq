use hooq::hooq;

#[hooq]
#[hooq::method(.map(|v| {
    println!("tag: {:?}", $tag);
    v
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
