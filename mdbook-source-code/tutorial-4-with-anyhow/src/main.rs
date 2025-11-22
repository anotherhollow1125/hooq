use anyhow::{Context, Result};
use hooq::hooq;

#[hooq]
#[hooq::method(.with_context(|| {
    let path = $path;
    let line = $line;
    let col = $col;
    let expr = ::hooq::summary!($source);

    format!("[{path}:{line}:{col}]\n{expr}")
}))]
fn display_name_by_mista(val: &toml::Value) -> Result<()> {
    let name = val.get("package")?.get("name")?.as_str()?;

    if name.contains("4") {
        return Err(anyhow::anyhow!(
            "name `{name}` contains '4'. Guido Mista disallow this."
        ));
    }

    println!("Mista「name: {name}」");

    Ok(())
}

#[hooq]
#[hooq::method(.with_context(|| {
    let path = $path;
    let line = $line;
    let col = $col;
    let expr = ::hooq::summary!($source);

    format!("[{path}:{line}:{col}]\n{expr}")
}))]
fn main() -> Result<()> {
    let path = std::env::args().nth(1).unwrap_or("Cargo.toml".to_string());

    let cargo_toml: toml::Value = toml::from_str(&std::fs::read_to_string(path)?)?;

    display_name_by_mista(&cargo_toml)?;

    Ok(())
}
