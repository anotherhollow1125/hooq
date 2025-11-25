use hooq::hooq;

#[hooq(my_flavor)]
fn display_name_by_mista(val: &toml::Value) -> Result<(), String> {
    // Method can be overridden by the one in flavor!
    #[hooq::method = my_flavor::ok_or_else]
    let name = val.get("package")?.get("name")?.as_str()?;

    if name.contains("4") {
        return Err(format!(
            "name `{name}` contains '4'. Guido Mista disallow this."
        ));
    }

    println!("Mista「name: {name}」");

    Ok(())
}

#[hooq(my_flavor)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args().nth(1).unwrap_or("Cargo.toml".to_string());

    let cargo_toml: toml::Value = toml::from_str(&std::fs::read_to_string(path)?)?;

    display_name_by_mista(&cargo_toml)?;

    Ok(())
}
