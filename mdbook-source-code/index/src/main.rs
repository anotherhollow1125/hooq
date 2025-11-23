use std::error::Error;

use hooq::hooq;

#[hooq]
fn load_host_and_port() -> Result<String, Box<dyn Error>> {
    // Loading APP_HOST
    let host = std::env::var("APP_HOST")?;

    // Loading APP_PORT
    let port = std::env::var("APP_PORT")?;

    // Convert to u16
    let port: u16 = port.parse()?;

    Ok(format!("{host}:{port}"))
}

#[hooq]
fn main() -> Result<(), Box<dyn Error>> {
    let host_and_port = load_host_and_port()?;

    println!("Server is running on: {}", host_and_port);

    // snip

    Ok(())
}

#[test]
fn test_load_host_and_port() {
    unsafe {
        std::env::set_var("APP_HOST", "localhost");
        std::env::set_var("APP_PORT", "8080");
    }
    let host_and_port = load_host_and_port().unwrap();
    assert_eq!(host_and_port, "localhost:8080");
}
