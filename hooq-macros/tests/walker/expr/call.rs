use hooq_macros::hooq;

#[hooq]
fn hoge() -> Result<(), ()> {
    Err(())
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

        Err(())
    })(hoge()?, hoge()?, 10)?;

    Err(())
}

#[test]
fn test() {
    func().unwrap_err();
}
