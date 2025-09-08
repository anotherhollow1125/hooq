#!/usr/bin/env -S cargo +nightly -q -Zscript run --release --manifest-path
---
[package]
name = "sync"
description = "README.md の生成およびhooqクレートのCargo.tomlにあるhooq-macrosとhooq-helpersのバージョンの同期を行うスクリプト"

[dependencies]
anyhow = "1.0.99"
toml = "0.9.5"
serde = { version = "1.0.219", features = ["derive"] }
handlebars = "6.3.2"
clap = { version = "4.5.47", features = ["derive"] }
hooq = { path = "../../hooq", features = ["anyhow"] }
---

use clap::Parser;
use hooq::hooq;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    check: bool,
}

#[derive(serde::Serialize)]
struct Readme {
    common: String,
}

#[hooq(anyhow)]
fn main() -> anyhow::Result<()> {
    let Args { check } = Args::parse();

    sync_sub_crate_versions(check)?;
    sync_readme(check)?;

    Ok::<(), anyhow::Error>(())
}

#[hooq(anyhow)]
fn sync_sub_crate_versions(check_only: bool) -> anyhow::Result<()> {
    let workspace_toml = toml::from_str::<toml::Value>(&std::fs::read_to_string("./Cargo.toml")?)?;

    if workspace_toml.as_table()?.get("workspace").is_none() {
        return Err(anyhow::anyhow!(
            "This script must be run in the workspace root."
        ));
    }

    let version = workspace_toml
        .as_table()?
        .get("workspace")?
        .as_table()?
        .get("package")?
        .as_table()?
        .get("version")?
        .as_str()?;

    let mut hooq_crate_toml =
        toml::from_str::<toml::Value>(&std::fs::read_to_string("./hooq/Cargo.toml")?)?;

    let macros_version_entry = hooq_crate_toml
        .as_table_mut()?
        .get_mut("dependencies")?
        .as_table_mut()?
        .get_mut("hooq-macros")?
        .as_table_mut()?
        .get_mut("version")?;

    if check_only && version != macros_version_entry.as_str()? {
        return Err(anyhow::anyhow!(
            "Version mismatch found in hooq-macros. Expected: {}, Found: {}",
            version,
            macros_version_entry.as_str()?
        ));
    } else if !check_only {
        *macros_version_entry = toml::Value::from(version);
    }

    let helpers_version_entry = hooq_crate_toml
        .as_table_mut()?
        .get_mut("dependencies")?
        .as_table_mut()?
        .get_mut("hooq-helpers")?
        .as_table_mut()?
        .get_mut("version")?;

    if check_only && version != helpers_version_entry.as_str()? {
        return Err(anyhow::anyhow!(
            "Version mismatch found in hooq-helpers. Expected: {}, Found: {}",
            version,
            helpers_version_entry.as_str()?
        ));
    } else if !check_only {
        *helpers_version_entry = toml::Value::from(version);
    }

    if !check_only {
        std::fs::write(
            "./hooq/Cargo.toml",
            toml::to_string_pretty(&hooq_crate_toml)?,
        )?;
    }

    Ok::<(), anyhow::Error>(())
}

#[hooq(anyhow)]
fn sync_readme(check_only: bool) -> anyhow::Result<()> {
    let common_md = std::fs::read_to_string("./hooq/README.md")?;

    let readme = Readme { common: common_md };

    let mut handlebars = handlebars::Handlebars::new();
    handlebars.set_strict_mode(true);
    handlebars.register_escape_fn(handlebars::no_escape);

    // GitHub README.md
    let github_template_md = std::fs::read_to_string("./docs/_readme_github.md.template")?;
    let github_readme_md = handlebars.render_template(&github_template_md, &readme)?;

    if check_only {
        let existing_github_readme_md = std::fs::read_to_string("./docs/README.md")?;
        if existing_github_readme_md != github_readme_md {
            return Err(anyhow::anyhow!(
                "README.md is out of date. Please run the sync script to update it."
            ));
        }
    } else {
        std::fs::write("./docs/README.md", github_readme_md)?;
    }

    // hooq-macros and hooq-helpers README.md
    let sub_crates_template_md = std::fs::read_to_string("./docs/_readme_sub_crates.md.template")?;
    let sub_crates_readme_md = handlebars.render_template(&sub_crates_template_md, &readme)?;

    for sub_crates in ["hooq-macros", "hooq-helpers"] {
        let path = format!("./{}/README.md", sub_crates);

        if check_only {
            let existing_sub_crate_readme_md = std::fs::read_to_string(&path)?;
            if existing_sub_crate_readme_md != sub_crates_readme_md {
                return Err(anyhow::anyhow!(
                    "{} is out of date. Please run the sync script to update it.",
                    path
                ));
            }
        } else {
            std::fs::write(&path, sub_crates_readme_md.clone())?;
        }
    }

    Ok::<(), anyhow::Error>(())
}
