use hooq::hooq;

#[hooq]
#[hooq::method(.ok_or_else(|| {
    format!("{} [Line: {}, {}]",
        stringify!($source),
        $line,
        $nth
    )
}))]
fn display_name(val: &toml::Value) -> Result<(), String> {
    let name = val.get("package")?.get("name")?.as_str()?;

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
