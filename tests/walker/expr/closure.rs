use hooq::hooq;

#[hooq]
fn minus(x: u32, y: u32) -> Result<u32, ()> {
    if y > x {
        return Err(());
    }
    Ok(x - y)
}

#[hooq]
fn two() -> Result<u32, ()> {
    Ok(2)
}

#[hooq]
#[hooq::method(.inspect(|_| {
    println!("tag: {:?}", $tag);
}))]
fn func() -> Result<(), ()> {
    #[hooq::tag("No Result Type (Block)")]
    let _ = |v| {
        minus(v, 1)?;
        minus(v, two()?)
    };

    #[hooq::tag("Result Type Annotated (Block)")]
    let _ = |v: u32| -> Result<u32, ()> {
        minus(v, 1)?;
        minus(v, two()?)
    };

    #[hooq::tag("Expr is Result Type (Block)")]
    let _ = |v| {
        minus(v, 1)?;
        minus(v, two()?)?;

        if v > 100 {
            return Err(());
        }

        Ok(minus(v, 3)? + 1)
    };

    #[hooq::tag("No Result Type (Single Expr)")]
    let _ = |v| minus(v, two()?);

    #[hooq::tag("Expr is Result Type (Single Expr)")]
    let _ = |v| {
        Ok({
            if v > 100 {
                return Err(());
            }

            minus(v, 1)? + minus(v, two()?)? + 1
        })
    };

    Ok(())
}

#[test]
fn test() {
    func().unwrap();
}
