use hooq::hooq;

#[hooq]
#[hooq::method(.$so_far.inspect(|_| {
    println!("Success: `{}` @ Line {}: Col: {}", stringify!($source), $line, $col);
}))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args().nth(1).unwrap_or("Cargo.toml".to_string());

    let _cargo_toml: toml::Value = toml::from_str(&std::fs::read_to_string(path)?)?;

    // snip

    Ok(())
}
