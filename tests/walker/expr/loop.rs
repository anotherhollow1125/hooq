use hooq::hooq;

#[hooq]
fn hoge() -> Result<(), ()> {
    Ok(())
}

#[hooq]
#[hooq::method(.inspect(|_| {
    println!("tag: {:?}", $tag);
}))]
fn func(flags: Vec<bool>) -> Result<(), ()> {
    let mut flags = flags.into_iter();

    loop {
        hoge()?;

        if flags.next().unwrap_or(false) {
            return Ok(());
        }

        if flags.next().unwrap_or(false) {
            return hoge();
        }

        // break に対してはフックしない仕様
        if flags.next().unwrap_or(false) {
            break Err(());
        }

        if flags.next().unwrap_or(false) {
            break Ok(hoge()?);
        }

        if flags.next().unwrap_or(false) {
            break hoge();
        }
    }?;

    Ok(())
}

#[test]
fn test() {
    func(vec![false, false, false, false, false, true]).unwrap();
}
