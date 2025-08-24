use hooq_macros::hooq;

#[hooq]
fn b() -> Result<u32, ()> {
    Ok(10)
}

#[hooq]
fn c() -> Result<u32, ()> {
    Ok(20)
}

#[hooq]
#[hooq::method(.inspect(|_| {
    println!("tag: {}", $tag);
}))]
fn func() -> Result<u32, ()> {
    #[hooq::tag("add")]
    let mut a = b()? + c()?;

    #[hooq::tag("add_assign")]
    {
        a += b()?;
    }

    Ok(a + b()? + c()?)
}

#[test]
fn test() {
    func().unwrap();
}
