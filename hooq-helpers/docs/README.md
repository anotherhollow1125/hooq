This is sub-crate for [hooq](../hooq) crate. Please use [hooq](../hooq) crate instead of using this crate directly.

# hooq

A simple macro that inserts a method before the `?` operator

The crate name comes from the acronym "HOOk for Question mark operator".

```rust
use hooq::hooq;

#[hooq]
#[hooq::method(.ok_or_else(|| format!("{} (L{}, {})", $expr_str, $line, $nth)))]
fn display_name(val: &toml::Value) -> Result<(), String> {
    let name = val.get("package")?.get("name")?.as_str()?;

    println!("name: {name}");

    #[hooq::skip]
    Ok(())
}

#[hooq]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cargo_toml = toml::from_str(&std::fs::read_to_string("Cargo.toml")?)?;

    display_name(&cargo_toml)?;

    Ok(())
}
```

The above expands into the following.

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
                    let line = 16usize;
                    {
                        ::std::io::_eprint(
                            format_args!("[{0}:L{1}] {2:?}\n", path, line, e),
                        );
                    };
                })?,
        )
        .inspect_err(|e| {
            let path = "/path/to/your/project/src/main.rs";
            let line = 16usize;
            {
                ::std::io::_eprint(format_args!("[{0}:L{1}] {2:?}\n", path, line, e));
            };
        })?;
    display_name(&cargo_toml)
        .inspect_err(|e| {
            let path = "/path/to/your/project/src/main.rs";
            let line = 18usize;
            {
                ::std::io::_eprint(format_args!("[{0}:L{1}] {2:?}\n", path, line, e));
            };
        })?;
    Ok(())
        .inspect_err(|e| {
            let path = "/path/to/your/project/src/main.rs";
            let line = 20usize;
            {
                ::std::io::_eprint(format_args!("[{0}:L{1}] {2:?}\n", path, line, e));
            };
        })
}
```

## Quick Start

Add the crate with `cargo add` as shown below, or

```bash
cargo add hooq
```

add it to your `Cargo.toml`.

```toml
[dependencies]
hooq = "*"
```

## Methods Inserted by Default

If nothing is specifically specified for `#[hooq]`, the following method is inserted:

```ignore
.inspect_err(|e| {
    let path = $path;
    let line = $line;

    ::std::eprintln!("[{path}:L{line}] {e:?}");
})
```

You can switch the hooked method with the inert attribute `#[hooq::method(...)]`. Also, if you specify a flavor like `#[hooq(log)]` or `#[hooq(anyhow)]` (requires the `anyhow` feature) at the macro call site, the inserted method will correspond to that flavor.

Available flavors can be found here: [hooq-macros/src/impls/flavor/presets/](../../hooq-macros/src/impls/flavor/presets/)

(More to come!)

## Attributes

(WIP)

## Metavariables

(WIP)

## Flavors

(WIP)
