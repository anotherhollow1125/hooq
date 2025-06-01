use hooq::hooq;

fn a() -> Result<bool, ()> {
    Ok(true)
}

fn b() -> Result<bool, ()> {
    Ok(false)
}

#[hooq]
#[hooq::method(.map(|v| {
    println!("tag: {:?}", $tag);
    v
}))]
fn func() -> Result<(), ()> {
    #[hooq::tag("closure")]
    let f = || {
        #[hooq::tag("inner")]
        Ok(())
    };

    #[hooq::tag("closure 2")]
    let g = || {
        println!("beep");

        true
    };

    #[hooq::tag("in eq binary ops exprs")]
    {
        let res = a()? == b()?;
        println!("{:?}", res);
    }

    let _ = g();

    #[hooq::tag("tail expr")]
    f()
}

#[test]
fn test() {
    func().unwrap();
}
