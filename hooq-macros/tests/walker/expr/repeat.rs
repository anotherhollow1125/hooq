use hooq_macros::hooq;

#[hooq]
fn hoge() -> Result<usize, ()> {
    Ok(5)
}

#[hooq]
#[hooq::method(.inspect(|_| {
    println!("tag: {}", $tag);
}))]
#[hooq::tag = "(no tag)"]
fn func() -> Result<(), ()> {
    let _ = [hoge()?; {
        fn _f() -> Result<(), ()> {
            hoge()?;

            Err(())
        }

        5
    }];

    Err(())
}

#[test]
fn test() {
    func().unwrap_err();
}
