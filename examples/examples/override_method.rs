use hooq::hooq;

#[hooq]
#[hooq::method(.map(|v| {
    println!("original");
    v
} ))]
fn hoge(b: bool) -> Result<(), ()> {
    if b {
        #[hooq::method(.map(|v| {
            println!("inner");
            v
        }))]
        return Ok(());
    }

    Ok(())
}

fn main() -> Result<(), ()> {
    hoge(true)?;
    hoge(false)?;

    Ok(())
}
