use hooq::hooq;

#[hooq(anyhow)]
fn main() -> anyhow::Result<()> {
    Err(anyhow::anyhow!("Hello, world!"))
}
