use hooq_macros::hooq;

#[hooq]
fn hoge() -> Result<(), ()> {
    Ok(())
}

#[hooq]
#[hooq::method(.inspect(|_| {
    println!("tag: {}", $tag);
}))]
#[hooq::tag = "(no tag)"]
#[hooq::tail_expr_idents("Ok", "Err")]
fn func() -> Result<(), ()> {
    (|_, _, _| {
        hoge()?;

        Ok(())
    })(hoge()?, hoge()?, 10)?;

    Ok(())
}

#[test]
fn test() {
    func().unwrap();
}
