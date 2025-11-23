#!/usr/bin/env -S cargo +nightly -q -Zscript run --release --manifest-path
---
[package]
name = "build_script"
description = "main.rs をビルドするためのビルドスクリプト"

[dependencies]
clap = { version = "4.5.49", features = ["derive"] }
hooq = { path = "../../../../hooq", features = ["anyhow"] }
anyhow = "1.0.100"
---

use anyhow::Result;
use clap::{Parser, ValueEnum};
use hooq::hooq;
use std::process::Command;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value_t = Mode::ExpandCheck)]
    mode: Mode,

    #[arg(short, long, default_value_t = BuildProfile::Debug)]
    profile: BuildProfile,
}

#[derive(ValueEnum, Clone, Copy, Debug)]
enum Mode {
    Build,
    Expand,
    ExpandCheck,
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mode::Build => write!(f, "build"),
            Mode::Expand => write!(f, "expand"),
            Mode::ExpandCheck => write!(f, "expand-check"),
        }
    }
}

#[derive(ValueEnum, Clone, Copy, Debug)]
enum BuildProfile {
    Debug,
    Release,
}

impl std::fmt::Display for BuildProfile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuildProfile::Debug => write!(f, "debug"),
            BuildProfile::Release => write!(f, "release"),
        }
    }
}

const ORIGINAL_HOOQ_PATH: &str = "../../../../../hooq";
const SRC_NAME: &str = "main";
const TOOLCHAIN: &str = "nightly-2025-11-18";

#[hooq(anyhow)]
fn hooq_build(build_profile: BuildProfile) -> Result<()> {
    // build hooq
    let current_dir = std::env::current_dir()?;
    let target_dir = current_dir.join("target").to_string_lossy().to_string();
    let toolchain = format!("+{TOOLCHAIN}");
    let mut args = vec![&toolchain, "build", "--target-dir", &target_dir];

    if let BuildProfile::Release = build_profile {
        args.push("--release");
    }

    let ok = Command::new("cargo")
        .current_dir(ORIGINAL_HOOQ_PATH)
        .args(&args)
        .status()?
        .success();

    if !ok {
        return Err(anyhow::anyhow!("Failed to build hooq"));
    }

    Ok(())
}

struct Pathes {
    deps_path: String,
    libhooq_path: String,
}

#[hooq(anyhow)]
fn gen_pathes(build_profile: BuildProfile) -> Result<Pathes> {
    let build_type = match build_profile {
        BuildProfile::Debug => "debug",
        BuildProfile::Release => "release",
    };

    let deps_path = format!("./target/{build_type}/deps");
    let path_generator = |target, ext| -> Result<String> {
        let res = std::fs::read_dir(&deps_path)?
            .filter_map(|entry| #[hooq::skip_all] {
                let entry = entry.ok()?;
                let file_name = entry.file_name();
                let file_name = file_name.to_str()?;
                if file_name.starts_with(target) && file_name.ends_with(ext) {
                    Some(entry.path().to_string_lossy().to_string())
                } else {
                    None
                }
            })
            .next()?;

        Ok(res)
    };

    let libhooq_path = path_generator("libhooq-", ".rlib")?;

    Ok(Pathes {
        deps_path,
        libhooq_path,
    })
}

#[hooq(anyhow)]
fn main_build(build_profile: BuildProfile) -> Result<()> {
    let Pathes {
        deps_path,
        libhooq_path,
    } = gen_pathes(build_profile)?;

    let ok = Command::new("rustup")
        .args([
            "run",
            TOOLCHAIN,
            "rustc",
            "--edition=2024",
            &format!("-Ldependency={deps_path}"),
            "--extern",
            &format!("hooq={libhooq_path}"),
            &format!("{SRC_NAME}.rs"),
            "-o",
            SRC_NAME,
        ])
        .status()?
        .success();

    if !ok {
        return Err(anyhow::anyhow!("Failed to build main"));
    }

    Ok(())
}

#[hooq(anyhow)]
fn main_expand(build_profile: BuildProfile) -> Result<PathBuf> {
    let Pathes {
        deps_path,
        libhooq_path,
    } = gen_pathes(build_profile)?;

    let bytes = Command::new("rustup")
        .args([
            "run",
            TOOLCHAIN,
            "rustc",
            "--edition=2024",
            &format!("-Ldependency={deps_path}"),
            "--extern",
            &format!("hooq={libhooq_path}"),
            "-Zunpretty=expanded",
            &format!("{SRC_NAME}.rs"),
        ])
        .output()?
        .stdout;

    let tmp_file = format!("{SRC_NAME}_tmp.expanded.rs").into();

    std::fs::write(&tmp_file, bytes)?;

    Ok(tmp_file)
}

#[hooq(anyhow)]
fn rustfmt(path: &Path) -> Result<()> {
    let status = Command::new("rustup")
        .args(["run", TOOLCHAIN, "rustfmt", "--edition=2024", path.to_str()?])
        .status()?;

    if !status.success() {
        return Err(anyhow::anyhow!("rustfmt failed"));
    }

    Ok(())
}

#[hooq(anyhow)]
fn main() -> Result<()> {
    let Args { mode, profile } = Args::parse();

    hooq_build(profile)?;

    match mode {
        Mode::Build => {
            let res = main_build(profile);

            if res.is_err() {
                // retry once after cleaning
                std::fs::remove_dir_all("./target")?;
                hooq_build(profile)?;
                main_build(profile)?;
            }
        }
        Mode::Expand | Mode::ExpandCheck => {
            let tmp = main_expand(profile)?;

            rustfmt(&tmp)?;

            match mode {
                Mode::Build => unreachable!(),
                Mode::Expand => {
                    let dest = format!("{SRC_NAME}.expanded.rs");
                    std::fs::rename(&tmp, &dest)?;
                    println!("Expanded source written to {dest}");
                },
                Mode::ExpandCheck => {
                    let dest = format!("{SRC_NAME}.expanded.rs");
                    let expected = std::fs::read_to_string(&dest).unwrap_or_default();
                    let actual = std::fs::read_to_string(&tmp)?;

                    if expected != actual {
                        return Err(anyhow::anyhow!(
                            "{dest} is out of date. Please run the build script to update it.
expected:
```rust
{expected}
```

actual:
```rust
{actual}
```"
                        ));
                    } else {
                        std::fs::remove_file(&tmp)?;
                        println!("{dest} is up to date.");
                    }
                }
            }
        }
    }

    Ok(())
}
