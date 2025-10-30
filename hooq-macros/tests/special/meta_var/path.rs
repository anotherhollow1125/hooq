use hooq_macros::hooq;

#[hooq]
#[hooq::method(.inspect(|_| {
    eprintln!("path: {}", $path);
}))]
fn func() -> Result<(), ()> {
    Err(())
}

#[test]
fn test() {
    func().unwrap_err();
}
