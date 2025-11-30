use hooq::hooq;

#[hooq(eyre)]
fn func1() -> eyre::Result<i32> {
    Err(eyre::eyre!("Error in func1"))
}

#[hooq(eyre)]
fn func2() -> eyre::Result<i32> {
    let res = func1()?;

    println!("{res}");

    Ok(res)
}

#[hooq(eyre)]
fn main() -> eyre::Result<()> {
    func2()?;

    Ok(())
}
