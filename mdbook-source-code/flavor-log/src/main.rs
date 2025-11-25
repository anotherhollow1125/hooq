use hooq::hooq;

#[hooq(log)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    Err("Hello, world!".into())
}
