use hooq::hooq;

fn display_name(val: &toml::Value) -> Result<(), String> {
    let name = val
        .get("package")
        .ok_or_else(|| format!("get package [Line: {}]", line!()))?
        .get("name")
        .ok_or_else(|| format!("get name [Line: {}]", line!()))?
        .as_str()
        .ok_or_else(|| format!("as_str [Line: {}]", line!()))?;

    println!("name: {name}");

    Ok(())
}

#[hooq]
#[hooq::method(.$so_far.inspect(|_| {
    println!("Success: `{}` @ Line {}: Col: {}", stringify!($source), $line, $col);
}))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args().nth(1).unwrap_or("Cargo.toml".to_string());

    let cargo_toml: toml::Value = toml::from_str(&std::fs::read_to_string(path)?)?;

    display_name(&cargo_toml)?;

    Ok(())
}
