use hooq_macros::hooq;

#[hooq]
fn hoge() -> Result<usize, ()> {
    Ok(5)
}

#[hooq]
#[hooq::method(.inspect(|_| {
    println!("tag: {}", $tag);
}))]
fn func() -> Result<(), ()> {
    let _ = [hoge()?; {
        fn _f() -> Result<(), ()> {
            hoge()?;

            Ok(())
        }

        5
    }];

    Ok(())
}

#[test]
fn test() {
    func().unwrap();
}
