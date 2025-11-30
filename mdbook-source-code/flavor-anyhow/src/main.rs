use hooq::hooq;

#[hooq(anyhow)]
fn func1() -> anyhow::Result<i32> {
    Err(anyhow::anyhow!("Error in func1"))
}

#[hooq(anyhow)]
fn func2() -> anyhow::Result<i32> {
    let res = func1()?;

    println!("{res}");

    Ok(res)
}

#[hooq(anyhow)]
fn main() -> anyhow::Result<()> {
    func2()?;

    Ok(())
}
