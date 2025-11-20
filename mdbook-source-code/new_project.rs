#!/usr/bin/env -S cargo +nightly -q -Zscript run --release --manifest-path
---
[dependencies]
hooq = { path = "../hooq" }
clap = { version = "4.5.53" , features = ["derive"] }
anyhow = "1.0.100"
toml_edit = "0.23.7"
---

use clap::Parser;
use hooq::hooq;
use toml_edit::{DocumentMut, InlineTable, Table, Value};

#[derive(Parser, Debug)]
struct Args {
    /// Name of the project
    project_name: String,
}

#[hooq(anyhow)]
fn main() -> anyhow::Result<()> {
    let Args { project_name } = Args::parse();

    cargo_new(&project_name)?;

    edit_cargo_toml(&project_name)?;

    create_tests_dir(&project_name)?;

    Ok(())
}

#[hooq(anyhow)]
fn cargo_new(project_name: &str) -> anyhow::Result<()> {
    let status = std::process::Command::new("cargo")
        .args(["new", project_name])
        .status()?;

    if !status.success() {
        return Err(anyhow::anyhow!("Failed to create project: {status}"));
    }

    Ok(())
}

#[hooq(anyhow)]
fn edit_cargo_toml(project_name: &str) -> anyhow::Result<()> {
    let cargo_toml_path = format!("{}/Cargo.toml", project_name);

    let cargo_toml_content = std::fs::read_to_string(&cargo_toml_path)?;

    let mut doc = cargo_toml_content.parse::<DocumentMut>()?;

    let dependencies = doc.get_mut("dependencies")?.as_table_like_mut()?;

    let mut hooq = InlineTable::new();

    hooq.insert("path", Value::from("../../hooq"));

    dependencies.insert("hooq", hooq.into());

    let mut dev_dependencies = Table::new();

    dev_dependencies.insert("insta", Value::from("1.44.0").into());

    doc.insert("dev-dependencies", dev_dependencies.into());

    std::fs::write(cargo_toml_path, doc.to_string())?;

    Ok(())
}

#[hooq(anyhow)]
fn create_tests_dir(project_name: &str) -> anyhow::Result<()> {
    let tests_dir_path = format!("{}/tests", project_name);
    std::fs::create_dir_all(&tests_dir_path)?;

    let test = format!(
        r#"#[test]
fn snapshot_test() {{
    let output = std::process::Command::new("cargo")
        .args(["run", "-q"])
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    insta::assert_snapshot!("{project_name}", format!(
        "STDOUT:\n{{}}\nSTDERR:\n{{}}",
        stdout, stderr
    ));
}}
"#
    );

    std::fs::write(format!("{}/test.rs", tests_dir_path), &test)?;

    Ok(())
}
