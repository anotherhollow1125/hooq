This is sub-crate for [hooq](../hooq) crate. Please use [hooq](../hooq) crate instead of using this crate directly.

<div align="center">
<a href="https://docs.rs/hooq/0.1.2/hooq/" target="_blank">
<img src="https://raw.githubusercontent.com/anotherhollow1125/hooq/refs/heads/main/assets/hooq_eye_catch3.png" />
</a>
<h1>hooq</h1>

<h3>A simple macro that inserts a method before `?`.</h3>

[![crate](https://img.shields.io/crates/v/hooq)](https://crates.io/crates/hooq)
[![docs](https://img.shields.io/docsrs/hooq/0.1.2)](https://docs.rs/hooq/0.1.2/hooq/)
[![Rust](https://github.com/anotherhollow1125/hooq/actions/workflows/rust.yml/badge.svg)](https://github.com/anotherhollow1125/hooq/actions/workflows/rust.yml)

🪝 The name hooq comes from combining 'HOOk' and the 'Question mark operator ( ? )'. 🪝

</div>

Documentations:
- tutorial: (mdBook coming soon.)
- docs.rs: <https://docs.rs/hooq/0.1.2/hooq/>

> [!NOTE]
> 日本語版ドキュメントはこちら: [docs/ja/README.md](https://github.com/anotherhollow1125/hooq/tree/main/docs/ja)

<hr />

```rust
use hooq::hooq;

#[hooq]
#[hooq::method(.ok_or_else(|| format!("{} (L{}, {})", $expr_str, $line, $nth)))]
fn display_name(val: &toml::Value) -> Result<(), String> {
    let name = val.get("package")?.get("name")?.as_str()?;

    #[hooq::skip_all]
    if name.contains("4") {
        return Err("name contains '4'. Guido Mista disallow this.".to_string());
    }

    println!("name: {name}");

    Ok(())
}

#[hooq]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cargo_toml = toml::from_str(&std::fs::read_to_string("Cargo.toml")?)?;

    display_name(&cargo_toml)?;

    Ok(())
}
```

The above expands to the following.

```ignore
use hooq::hooq;
fn display_name(val: &toml::Value) -> Result<(), String> {
    let name = val
        .get("package")
        .ok_or_else(|| ::alloc::__export::must_use({
            ::alloc::fmt::format(
                format_args!("{0} (L{1}, {2})", "val.get(\"package\")", 6usize, "1st ?"),
            )
        }))?
        .get("name")
        .ok_or_else(|| ::alloc::__export::must_use({
            ::alloc::fmt::format(
                format_args!(
                    "{0} (L{1}, {2})", "val.get(\"package\") ? .get(\"name\")", 6usize,
                    "2nd ?",
                ),
            )
        }))?
        .as_str()
        .ok_or_else(|| ::alloc::__export::must_use({
            ::alloc::fmt::format(
                format_args!(
                    "{0} (L{1}, {2})",
                    "val.get(\"package\") ? .get(\"name\") ? .as_str()", 6usize, "3rd ?",
                ),
            )
        }))?;
    if name.contains("4") {
        return Err("name contains '4'. Guido Mista disallow this.".to_string());
    }
    {
        ::std::io::_print(format_args!("name: {0}\n", name));
    };
    Ok(())
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cargo_toml = toml::from_str(
            &std::fs::read_to_string("Cargo.toml")
                .inspect_err(|e| {
                    let path = "/path/to/your/project/src/main.rs";
                    let line = 20usize;
                    {
                        ::std::io::_eprint(
                            format_args!("[{0}:L{1}] {2:?}\n", path, line, e),
                        );
                    };
                })?,
        )
        .inspect_err(|e| {
            let path = "/path/to/your/project/src/main.rs";
            let line = 20usize;
            {
                ::std::io::_eprint(format_args!("[{0}:L{1}] {2:?}\n", path, line, e));
            };
        })?;
    display_name(&cargo_toml)
        .inspect_err(|e| {
            let path = "/path/to/your/project/src/main.rs";
            let line = 22usize;
            {
                ::std::io::_eprint(format_args!("[{0}:L{1}] {2:?}\n", path, line, e));
            };
        })?;
    Ok(())
}
```

## Install

Add it with `cargo add` as shown below,

```bash
cargo add hooq
```

or add it to your `Cargo.toml`.

```toml
[dependencies]
hooq = "0.1.2"
```

## Method inserted by default

If you don't specify anything for `#[hooq]`, the following method is inserted by default.

```ignore
.inspect_err(|e| {
    let path = $path;
    let line = $line;

    ::std::eprintln!("[{path}:L{line}] {e:?}");
})
```

You can switch the method to hook with the inert attribute `#[hooq::method(...)]`. Also, when you specify a flavor at the call site such as `#[hooq(log)]` or `#[hooq(anyhow)]` (the `anyhow` feature is required), the inserted method will change according to that flavor!

You can find the available flavors here: [hooq-macros/src/impls/flavor/presets/](https://github.com/anotherhollow1125/hooq/tree/main/hooq-macros/src/impls/flavor/presets)

(More to come!)

## Attributes

(WIP)

## Meta variables

(WIP)

## Flavor

(WIP)
