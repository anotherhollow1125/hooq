This is sub-crate for [hooq](../hooq) crate. Please use [hooq](../hooq) crate instead of using this crate directly.

# hooq

`?` 前にメソッドを挿入するシンプルなマクロ

クレート名の由来は "HOOk for Question mark operator" のアクロニム

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

上記が以下のように展開されます。

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

以下に示すように `cargo add` で加えるか、

```bash
cargo add hooq
```

`Cargo.toml` に加えてください。

```toml
[dependencies]
hooq = "0.1.0"
```

## デフォルトで挿入されるメソッド

`#[hooq]` として特に指定しなければ次のメソッドが挿入されます。

```ignore
.inspect_err(|e| {
    let path = $path;
    let line = $line;

    ::std::eprintln!("[{path}:L{line}] {e:?}");
})
```

`#[hooq::method(...)]` 不活性アトリビュートでフックするメソッドを切り替えられる他、マクロ呼び出し部を `#[hooq(log)]` や `#[hooq(anyhow)]` ( `anyhow` feature が必要 ) としてFlavorを指定した場合などは、そのFlavorにちなんだメソッドになります！

用意されている Flavor は次のディレクトリから見られます: [hooq-macros/src/impls/flavor/presets/](https://github.com/anotherhollow1125/hooq/tree/main/hooq-macros/src/impls/flavor/presets)

(今後拡充予定です！)

## アトリビュート

(WIP)

## メタ変数

(WIP)

## Flavor

(WIP)
