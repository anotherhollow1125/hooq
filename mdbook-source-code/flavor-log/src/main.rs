use hooq::hooq;

#[hooq(log)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::new()
        .format_timestamp(None)
        .filter_level(log::LevelFilter::Error)
        .init();

    Err("Hello, world!".into())
}
