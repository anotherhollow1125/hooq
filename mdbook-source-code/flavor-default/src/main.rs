use hooq::hooq;

#[hooq]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    Err("Hello, world!".into())
}
