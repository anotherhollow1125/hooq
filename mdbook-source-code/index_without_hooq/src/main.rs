use std::error::Error;

fn load_host_and_port() -> Result<String, Box<dyn Error>> {
    // APP_HOST の読み込み
    let host = std::env::var("APP_HOST")?;

    // APP_PORT の読み込み
    let port = std::env::var("APP_PORT")?;

    // u16 への変換
    let port: u16 = port.parse()?;

    Ok(format!("{host}:{port}"))
}

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
