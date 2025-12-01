#!/usr/bin/env -S cargo +nightly -q -Zscript run --release --manifest-path
---
[dependencies]
toml_edit = "0.23.7"
hooq = { path = "../hooq" }
anyhow = "1.0.100"
handlebars = "6.3.2"
serde = { version = "1.0.228", features = ["derive"] }
---

use hooq::hooq;

const LATEST: &str = "v0.3.0";
const VERSIONS: &[&str] = &["v0.3.0"];

#[hooq(anyhow)]
fn assert_dir() -> anyhow::Result<()> {
    let dir = std::env::current_dir()?;

    if dir.ends_with("docs") {
        Ok(())
    } else {
        Err(anyhow::anyhow!(
            "This script must be run in the 'docs' directory"
        ))
    }
}

#[hooq(anyhow)]
fn build_index() -> anyhow::Result<()> {
    let mut handlebars = handlebars::Handlebars::new();
    handlebars.set_strict_mode(true);
    handlebars.register_escape_fn(handlebars::no_escape);

    handlebars.register_template_string("index", HTML)?;

    #[derive(serde::Serialize)]
    struct Context {
        versions: &'static [&'static str],
    }

    let context = Context { versions: VERSIONS };
    let rendered = handlebars.render("index", &context)?;

    std::fs::write("./books/index.html", rendered)?;

    Ok(())
}

#[hooq(anyhow)]
fn build_books(version: &str, is_latest: bool) -> anyhow::Result<()> {
    let status = std::process::Command::new("git")
        .args(["switch", "-d", version])
        .status()?;

    if !status.success() {
        return Err(anyhow::anyhow!("Failed to switch to branch {}", version));
    }

    std::fs::create_dir_all(format!("./books/{}/en", version))?;

    fn build_books_inner(lang: &str, version: &str) -> anyhow::Result<()> {
        // edit book.toml
        let book_toml_path = format!("./{}/hooq-mdbook-{}/book.toml", lang, lang);
        let book_toml_str = std::fs::read_to_string(&book_toml_path)?;
        let mut book_toml: toml_edit::DocumentMut = book_toml_str.parse()?;
        book_toml
            .as_table_mut()
            .get_mut("output")?
            .as_table_mut()?
            .get_mut("html")?
            .as_table_mut()?
            .insert(
                "site-url",
                toml_edit::value(format!("/hooq/{}/{}", version, lang)),
            );

        // build mdBook
        let status = std::process::Command::new("mdbook")
            .args([
                "build",
                &format!("./{}/hooq-mdbook-{}", lang, lang),
                "-d",
                &format!("./books/{}/{}", version, lang),
            ])
            .status()?;

        if !status.success() {
            return Err(anyhow::anyhow!(
                "Failed to build mdBook for version {} and language {}",
                version,
                lang
            ));
        }

        Ok(())
    }

    for lang in &["en", "ja"] {
        build_books_inner(lang, version)?;

        if is_latest {
            build_books_inner(lang, "latest")?;
        }
    }

    Ok(())
}

#[hooq(anyhow)]
fn main() -> anyhow::Result<()> {
    assert_dir()?;

    std::fs::remove_dir_all("books")?;
    std::fs::create_dir("books")?;

    build_index()?;

    for version in VERSIONS {
        let is_latest = *version == LATEST;
        build_books(version, is_latest)?;
    }

    Ok(())
}

const HTML: &'static str = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>hooq mdBooks</title>
    <style>
        .title {
            display: flex;
            align-items: center;
            gap: 12px;
            margin: 16px 0;
        }
        .title img {
            height: 48px;
            width: auto;
        }
        @media (max-width: 480px) {
            .title img { height: 40px; }
        }
    </style>
</head>
<body>
    <div class="title">
        <img src="https://raw.githubusercontent.com/anotherhollow1125/hooq/refs/heads/main/assets/hooq_logo.svg" alt="hooq logo" width="48" height="48" decoding="async" />
        <h1>hooq mdBooks</h1>
    </div>
    <p>Latests:</p>
    <ul>
        <li><a href="latest/en/">hooq Reference (latest)</a></li>
        <li><a href="latest/ja/">hooq Reference 日本語版 (latest)</a></li>
    </ul>
    <p>All versions:</p>
    <ul>
        {{#each versions}}
        <li><a href="{{this}}/en/">hooq Reference ({{this}})</a></li>
        <li><a href="{{this}}/ja/">hooq Reference 日本語版 ({{this}})</a></li>
        {{/each}}
    </ul>
</body>
</html>
"#;
