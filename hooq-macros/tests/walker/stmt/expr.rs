use hooq_macros::hooq;

fn a() -> Result<bool, ()> {
    Ok(true)
}

fn b() -> Result<bool, ()> {
    Ok(false)
}

#[hooq]
#[hooq::method(.inspect(|_| {
    println!("tag: {}", $tag);
}))]
#[hooq::tail_expr_idents("Ok", "Err")]
fn func() -> Result<(), ()> {
    #[hooq::tag = "closure"]
    let f = || {
        #[hooq::tag = "inner"]
        Err(())
    };

    #[hooq::tag = "closure 2"]
    let g = || {
        println!("beep");

        true
    };

    #[hooq::tag = "in eq binary ops exprs"]
    {
        let _ = a()? == b()?;
    }

    let _ = g();

    #[hooq::tag = "tail expr"]
    f()
}

#[test]
fn test() {
    func().unwrap_err();
}
