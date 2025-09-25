use hooq_macros::hooq;

#[hooq]
fn hoge() -> Result<bool, ()> {
    Ok(true)
}

#[hooq]
#[hooq::method(.inspect(|_| {
    println!("tag: {}", $tag);
}))]
#[hooq::tag = "(no tag)"]
#[hooq::tail_expr_idents("Ok", "Err")]
fn func(flag: bool) -> Result<(), ()> {
    if hoge()? {
        let _ = hoge()?;

        if flag {
            return Err(());
        }

        if false {
            return hoge().map(|_| ());
        }

        Err(())
    } else if hoge()? {
        let _ = hoge()?;

        Err(())
    } else {
        let _ = hoge()?;

        Err(())
    }
}

#[test]
fn test() {
    func(true).unwrap_err();
}
