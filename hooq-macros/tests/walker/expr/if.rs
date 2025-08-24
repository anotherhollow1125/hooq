use hooq_macros::hooq;

#[hooq]
fn hoge() -> Result<bool, ()> {
    Ok(true)
}

#[hooq]
#[hooq::method(.inspect(|_| {
    println!("tag: {}", $tag);
}))]
fn func(flag: bool) -> Result<(), ()> {
    if hoge()? {
        let _ = hoge()?;

        if flag {
            return Ok(());
        }

        if false {
            return hoge().map(|_| ());
        }

        Ok(())
    } else if hoge()? {
        let _ = hoge()?;

        Ok(())
    } else {
        let _ = hoge()?;

        Err(())
    }
}

#[test]
fn test() {
    func(true).unwrap();
}
