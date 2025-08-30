#[hooq::hooq]
fn enresult<T>(v: T) -> Result<T, ()> {
    Ok(v)
}

#[hooq::hooq]
#[hooq::method(.inspect(|_| {
    println!("expr: {}, tag: {}", $expr, $tag);
}))]
#[hooq::tag = "main"]
fn main() -> Result<(), ()> {
    let v = vec![
        #[hooq::tag = "first item"]
        enresult(1_u32)?,
        #[hooq::tag = "second item"]
        enresult(2_u32)?,
    ];

    #[hooq::tag = "println"]
    println!("{:?}", enresult(v)?);

    Ok(())
}
