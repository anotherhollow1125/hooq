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
fn func(flag: bool) -> Result<(), ()> {
    let _: ((), usize, Result<(), ()>) = (
        hoge()?,
        {
            if !flag {
                return Err(());
            }

            10
        },
        {
            if flag {
                return hoge();
            }

            Err(())
        },
    );

    Err(())
}

#[test]
fn test() {
    func(true).unwrap_err();
}
