#!/usr/bin/env -S cargo +nightly -q -Zscript run --release --manifest-path
---
[dependencies]
hooq = { path = "../hooq" }
clap = { version = "4.5.53" , features = ["derive"] }
anyhow = "1.0.100"
toml_edit = "0.23.7"
---

use clap::{Parser, ValueEnum};
use hooq::hooq;
use toml_edit::{DocumentMut, InlineTable, Table, Value};

#[derive(ValueEnum, Debug, Clone, Copy)]
enum Mode {
    StdOutErr,
    Expand,
}

#[derive(Parser, Debug)]
struct Args {
    /// Name of the project
    project_name: String,
    /// Mode of testing
    #[clap(short, long, value_enum, default_value_t = Mode::StdOutErr)]
    mode: Mode,
}

#[hooq(anyhow)]
fn main() -> anyhow::Result<()> {
    let Args { project_name, mode } = Args::parse();

    cargo_new(&project_name)?;

    edit_cargo_toml(&project_name, mode)?;

    match mode {
        Mode::StdOutErr => create_tests_dir_for_std_out_err(&project_name)?,
        Mode::Expand => create_tests_dir_for_expand(&project_name)?,
    }

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
fn edit_cargo_toml(project_name: &str, mode: Mode) -> anyhow::Result<()> {
    let cargo_toml_path = format!("{}/Cargo.toml", project_name);

    let cargo_toml_content = std::fs::read_to_string(&cargo_toml_path)?;

    let mut doc = cargo_toml_content.parse::<DocumentMut>()?;

    let dependencies = doc.get_mut("dependencies")?.as_table_like_mut()?;

    let mut hooq = InlineTable::new();

    hooq.insert("path", Value::from("../../hooq"));

    dependencies.insert("hooq", hooq.into());

    let mut dev_dependencies = Table::new();

    match mode {
        Mode::StdOutErr => {
            dev_dependencies.insert("insta", Value::from("1.44.0").into());
        }
        Mode::Expand => {
            let mut test_helpers = InlineTable::new();
            test_helpers.insert("path", Value::from("../../test-helpers"));

            dev_dependencies.insert("test-helpers", test_helpers.into());
        }
    }

    doc.insert("dev-dependencies", dev_dependencies.into());

    std::fs::write(cargo_toml_path, doc.to_string())?;

    Ok(())
}

#[hooq(anyhow)]
fn create_tests_dir_for_std_out_err(project_name: &str) -> anyhow::Result<()> {
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

    insta::assert_snapshot!(
        "{project_name}",
        format!("STDOUT:\n{{}}\nSTDERR:\n{{}}", stdout, stderr)
    );
}}
"#
    );

    std::fs::write(format!("{}/test.rs", tests_dir_path), &test)?;

    Ok(())
}

#[hooq(anyhow)]
fn create_tests_dir_for_expand(project_name: &str) -> anyhow::Result<()> {
    let tests_dir_path = format!("{}/tests", project_name);
    std::fs::create_dir_all(&tests_dir_path)?;

    let test = format!(
        r#"use test_helpers::MaskMode::*;
use test_helpers::mask_project_root;

#[test]
fn snapshot_test() {{
    mask_project_root(".", UnMask);

    let pre_expanded = std::fs::read_to_string("./src/main.expanded.rs").ok();

    let output = String::from_utf8_lossy(
        &std::process::Command::new("cargo")
            .arg("expand")
            .output()
            .unwrap()
            .stdout,
    )
    .to_string();

    if let Some(pre_expanded) = pre_expanded
        && pre_expanded != output
    {{
        panic!(
            "snapshot test failed: \n\n--- pre expanded ---\n{{pre_expanded}}\n\n--- new expanded ---\n{{output}}\n"
        );
    }} else {{
        std::fs::write("./src/main.expanded.rs", output).unwrap();
    }}

    mask_project_root(".", Mask);
}}
"#
    );

    std::fs::write(format!("{}/test.rs", tests_dir_path), &test)?;

    Ok(())
}
