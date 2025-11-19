#!/usr/bin/env -S cargo +nightly -q -Zscript run --release --manifest-path
---
[package]
name = "sync"
description = "README.md の生成およびhooqクレートのCargo.tomlにあるhooq-macrosとhooq-helpersのバージョンの同期を行うスクリプト"

[dependencies]
anyhow = "1.0.99"
toml = "0.9.5"
serde = { version = "1.0.219", features = ["derive"] }
serde_json = "1.0.145"
handlebars = "6.3.2"
clap = { version = "4.5.47", features = ["derive"] }
hooq = { path = "../../hooq", features = ["anyhow"] }
reqwest = { version = "0.12.24", features = ["blocking", "json"] }
chrono = "0.4.42"
strum = { version = "0.27.2", features = ["derive"] }
---

use chrono::DateTime;
use clap::{Parser, ValueEnum};
use hooq::hooq;

#[derive(ValueEnum, Clone, Copy, Debug, strum::Display)]
enum Mode {
    Wip,
    Publish,
    AddTag,
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    check: bool,
    #[arg(short, long, value_enum, default_value_t = Mode::Wip)]
    mode: Mode,
}

#[derive(serde::Serialize)]
struct Readme {
    common: String,
}

#[derive(serde::Serialize)]
struct VersionInfo {
    version: String,
}

#[hooq(anyhow)]
fn get_published_version() -> anyhow::Result<String> {
    #[derive(serde::Deserialize, Debug)]
    struct CrateVersion {
        num: String,
        yanked: bool,
        created_at: String,
    }

    #[derive(serde::Deserialize, Debug)]
    struct CrateResponse {
        versions: Vec<CrateVersion>,
    }

    let resp: serde_json::Value = reqwest::blocking::Client::builder()
        .user_agent("hooq-sync-script/0.1.0")
        .build()?
        .get("https://crates.io/api/v1/crates/hooq/versions")
        .send()?
        .json()?;

    let resp: CrateResponse = serde_json::from_value(resp.clone())
        .map_err(|e| anyhow::anyhow!("Failed to parse response: {} from {}", e, resp))?;

    let latest_version = resp
        .versions
        .iter()
        .filter_map(|v| #[hooq::skip_all]
        {
            if v.yanked {
                return None;
            }

            let datetime = DateTime::parse_from_rfc3339(&v.created_at).ok()?;

            Some((v.num.clone(), datetime))
        })
        .max_by_key(|v| v.1)
        .ok_or_else(|| anyhow::anyhow!("No available versions found"))?;

    Ok(latest_version.0)
}

#[hooq(anyhow)]
fn main() -> anyhow::Result<()> {
    let Args { check, mode } = Args::parse();

    let current_version = sync_sub_crate_versions(check)?;
    let version = match mode {
        Mode::Wip => get_published_version()?,
        Mode::Publish | Mode::AddTag => current_version.clone(),
    };
    let version_info = VersionInfo { version };

    cargo_sort(check)?;
    for lang in ["", "/ja"] {
        sync_readme(lang, check, &version_info)?;
    }

    if let Mode::AddTag = mode
        && !check
    {
        git_tag(&current_version)?;
    }

    Ok::<(), anyhow::Error>(())
}

#[hooq(anyhow)]
fn sync_sub_crate_versions(check_only: bool) -> anyhow::Result<String> {
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

    Ok::<String, anyhow::Error>(version.to_string())
}

// TOMLの上書きは順番がおかしくなるので、cargo-sortで整形
#[hooq(anyhow)]
fn cargo_sort(check_only: bool) -> anyhow::Result<()> {
    use std::process::Command;

    let mut command = Command::new("cargo");
    let command = command.arg("sort").arg("--workspace");

    let command = if check_only {
        command.arg("--check")
    } else {
        command
    };

    let status = command.status()?;

    if !status.success() {
        return Err(anyhow::anyhow!("cargo sort failed"));
    }

    Ok::<(), anyhow::Error>(())
}

#[hooq(anyhow)]
fn sync_readme(lang: &str, check_only: bool, version_info: &VersionInfo) -> anyhow::Result<()> {
    let common_template_md =
        std::fs::read_to_string(format!("./docs{lang}/_readme_root.md.template"))?;

    let mut handlebars = handlebars::Handlebars::new();
    handlebars.set_strict_mode(true);
    handlebars.register_escape_fn(handlebars::no_escape);

    let common_md = handlebars.render_template(&common_template_md, &version_info)?;

    let readme = Readme { common: common_md };

    // GitHub README.md
    let github_template_md = std::fs::read_to_string("./docs/_readme_github.md.template")?;
    let github_readme_md = handlebars.render_template(&github_template_md, &readme)?;

    let path = format!("./docs{lang}/README.md");

    if check_only {
        let existing_github_readme_md = std::fs::read_to_string(&path)?;
        if existing_github_readme_md != github_readme_md {
            return Err(anyhow::anyhow!(
                "README.md is out of date. Please run the sync script to update it."
            ));
        }
    } else {
        std::fs::write(&path, github_readme_md)?;
    }

    // hooq README.md
    // コピーするのみ
    let hooq_readme_md = readme.common.clone();

    let path = format!("./hooq/docs{lang}/README.md");

    if check_only {
        let existing_hooq_readme_md = std::fs::read_to_string(&path)?;
        if existing_hooq_readme_md != hooq_readme_md {
            return Err(anyhow::anyhow!(
                "README.md is out of date. Please run the sync script to update it."
            ));
        }
    } else {
        std::fs::write(&path, hooq_readme_md)?;
    }

    // hooq-macros and hooq-helpers README.md
    let sub_crates_template_md = std::fs::read_to_string("./docs/_readme_sub_crates.md.template")?;
    let sub_crates_readme_md = handlebars.render_template(&sub_crates_template_md, &readme)?;

    for sub_crates in ["hooq-macros", "hooq-helpers"] {
        let path = format!("./{sub_crates}/docs{lang}/README.md");

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

#[hooq(anyhow)]
fn git_tag(current_version: &str) -> anyhow::Result<()> {
    let tag = format!("v{}", current_version);

    let status = std::process::Command::new("git")
        .arg("tag")
        .arg(&tag)
        .status()?;

    if !status.success() {
        return Err(anyhow::anyhow!("git tag failed"));
    }

    println!("Created git tag: {}", tag);

    Ok(())
}
