This is sub-crate for [hooq](https://github.com/anotherhollow1125/hooq/tree/main/hooq) crate. Please use [hooq](https://github.com/anotherhollow1125/hooq/tree/main/hooq) crate instead of using this crate directly.

<div align="center">
<a href="https://docs.rs/hooq/latest/hooq/" target="_blank">
<img src="https://raw.githubusercontent.com/anotherhollow1125/hooq/refs/heads/main/assets/hooq_eye_catch3.png" />
</a>
<h1>hooq</h1>

<h3>A simple macro that inserts (hooks) a method before question operator (`?`).</h3>

[![crate](https://img.shields.io/crates/v/hooq)](https://crates.io/crates/hooq)
[![docs](https://img.shields.io/docsrs/hooq/0.3.0)](https://docs.rs/hooq/0.3.0/hooq/)
[![Rust](https://github.com/anotherhollow1125/hooq/actions/workflows/rust.yml/badge.svg)](https://github.com/anotherhollow1125/hooq/actions/workflows/rust.yml)

?🪝 The name hooq comes from combining 'HOOk' and the 'Question mark operator ( ? )'. 🪝?

Enhance your questions by hooq!?

</div>

Keywords: `Result`, `Option`, `hook`, `Result hook`, `Option hook`, `? hook`, `question hook`, `error`, `logging`

Documentations:
- mdBooks: <https://anotherhollow1125.github.io/hooq/>
    - latest intro: <https://anotherhollow1125.github.io/hooq/latest/en/index.html>
    - latest tutorial: <https://anotherhollow1125.github.io/hooq/latest/en/tutorial/index.html>
    - latest reference: <https://anotherhollow1125.github.io/hooq/latest/en/reference/index.html>
- docs.rs: <https://docs.rs/hooq/latest/hooq/>
- deepwiki: <https://deepwiki.com/anotherhollow1125/hooq>
    - ⚠️ Please be careful of generative AI hallucination.

> [!NOTE]
> 日本語版ドキュメントはこちら: [docs/ja/README.md](https://github.com/anotherhollow1125/hooq/tree/main/docs/ja)

<hr />

Insert methods specified with `#[hooq::method(...)]` between expressions and the `?` operator (Question Operator)!

```rust
use hooq::hooq;

#[hooq]
#[hooq::method(.map(|v| v * 2))]
fn double(s: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let res = s.parse::<u32>()?;
    Ok(res)
}

fn double_expanded(s: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let res = s.parse::<u32>().map(|v| v * 2)?;
    Ok(res)
}

fn main() {
    assert_eq!(double("21").unwrap(), double_expanded("21").unwrap());
}
```

You don't have to explicitly specify `#[hooq::method(...)]`—there's also a mechanism called "flavors" for easily applying pre-configured settings!

## Why hooq?

Being able to insert a method before the `?` operator lets you reduce boilerplate for debugging and logging. As a result, you can increase logging information without sacrificing readability.

The [`Context::with_context`](https://docs.rs/anyhow/latest/anyhow/trait.Context.html#tymethod.with_context) method from the [anyhow](https://docs.rs/anyhow/latest/anyhow/) crate is a prime example. The hooq crate provides a flavor (anyhow) to make inserting this method easy.

```rust,should_panic
use hooq::hooq;

#[hooq(anyhow)]
fn func1() -> anyhow::Result<i32> {
    Err(anyhow::anyhow!("Error in func1"))
}

#[hooq(anyhow)]
fn func2() -> anyhow::Result<i32> {
    func1()
}

#[hooq(anyhow)]
fn main() -> anyhow::Result<()> {
    func2()?;

    Ok(())
}
```

`.with_context(|| {...})` gets hooked, and the output looks like this:

```plaintext
Error: [mdbook-source-code/flavor-anyhow/src/main.rs:15:12]
  15>    func2()?
    |

Caused by:
    0: [mdbook-source-code/flavor-anyhow/src/main.rs:10:5]
         10>    func1()
           |
    1: [mdbook-source-code/flavor-anyhow/src/main.rs:5:5]
          5>    Err(anyhow::anyhow!("Error in func1"))
           |
    2: Error in func1
```

Other concrete examples would make the README too long, so they have been compiled in the mdBook: [Why use hooq?](https://anotherhollow1125.github.io/hooq/latest/en/#why-use-hooq)

Applying `#[hooq]` (or `#[hooq(anyhow)]`) to all functions yields information close to an error stack trace.

Below is a comparison table with other ways to obtain stack-trace-like information.

|| [`Backtrace`](https://doc.rust-lang.org/std/backtrace/struct.Backtrace.html) | [`tracing`](https://docs.rs/tracing/latest/tracing) | `hooq` |
|:-|:-:|:-:|:-:|
| Learning cost & flexibility | ⚠️ | ⚠️ | 🌈 |
| Ease of type definitions | ⚠️ | ✅ | ✅ |
| Macro-less | 🌈 | ❌ | ❌ |
| Information control | ⚠️ | ✅ | 🌈 |
| Platform support | ⚠️ | ✅ | 🌈 |

Legend:
- 🌈: Excellent
- ✅: Good
- ⚠️: Not so good
- ❌: Poor

Explanations:
- Learning cost & flexibility
    - ⚠️ `Backtrace` requires defining the environment variable `RUST_LIB_BACKTRACE=1`, and because it depends on OS threads you need knowledge outside pure Rust control flow.
    - ⚠️ `tracing` can be overkill if your only goal is a stack-trace-like overview. If you are already comfortable with it, it is a reasonable option.
    - 🌈 `hooq` only needs attaching an attribute macro to function heads.
- Ease of type definitions
    - ⚠️ When combining `Backtrace` with crates like `thiserror`, you must include the backtrace field up front. The more granular your error types, the harder retrospective addition becomes, or the less simple the error representation stays.
    - ✅ `tracing` imposes no particular constraints here.
    - ✅ `hooq` works smoothly with any error handling crate.
- Macro-less
    - 🌈 `Backtrace` does not rely on macros, which feels lightweight.
    - ❌ To conveniently obtain stack-trace-equivalent info with `tracing`, using `#[tracing::instrument(err)]` (or similar) is almost mandatory.
    - ❌ `hooq` is an attribute macro crate; if you prefer to avoid macros entirely, it is not suitable.
- Information control
    - ⚠️ The raw output of `Backtrace` is often too verbose 😵. In async contexts raw frames can be nearly useless and frequently excessive.
        - Crates like [`color-eyre`](https://docs.rs/color-eyre/latest/color_eyre/) can improve formatting and make it more practical.
    - ✅ `tracing` can follow functions even across async boundaries; however, to know details such as “which line’s `?` operator?” you must add manual logging.
    - 🌈 `hooq` (similar to `#[tracing::instrument]`) traces only functions annotated with `#[hooq]`, giving you precise, opt‑in coverage. It can capture exact positions of `?`, `return`, and tail expressions for finer granularity.
        - Being an attribute macro you can conditionally attach it only in tests or behind features (`#[cfg_attr(..., hooq(...))]`).
        - 💡 Can be combined with `tracing` to increase information granularity. See the flavor docs for [`tracing`](https://anotherhollow1125.github.io/hooq/latest/en/reference/flavors.html#tracing) in the mdBook.
- Platform support
    - ⚠️ `Backtrace` may be unavailable or partial on some platforms (see official docs: https://doc.rust-lang.org/std/backtrace/index.html#platform-support).
    - ✅ Ordinary `tracing` logging use cases typically face few platform restrictions.
    - 🌈 `hooq` merely inserts a method before `?` (and optionally `return` / tail expressions), so it does not depend on platform-specific features. Creative usage per platform is possible.
        - 💡 For example `#[hooq::method(.unwrap()!)]` can make `?` behave akin to a forced unwrap alias.

## Documentation

For detailed usage instructions, please refer to the following! (Also included at the beginning, but repeated here)

- mdBooks: <https://anotherhollow1125.github.io/hooq/>
    - latest intro: <https://anotherhollow1125.github.io/hooq/latest/en/index.html>
    - latest tutorial: <https://anotherhollow1125.github.io/hooq/latest/en/tutorial/index.html>
    - latest reference: <https://anotherhollow1125.github.io/hooq/latest/en/reference/index.html>
- docs.rs: <https://docs.rs/hooq/latest/hooq/>
- deepwiki: <https://deepwiki.com/anotherhollow1125/hooq>
    - ⚠️ Please be careful of generative AI hallucination.

> [!NOTE]
> 日本語版ドキュメントはこちら: [docs/ja/README.md](https://github.com/anotherhollow1125/hooq/tree/main/docs/ja)

## Install

Add it with `cargo add` as shown below,

```bash
cargo add hooq
```

or add it to your `Cargo.toml`.

```toml
[dependencies]
hooq = "0.3.0"
```

> [!NOTE]
> [MSRV](https://doc.rust-lang.org/cargo/reference/rust-version.html) is [1.88](https://blog.rust-lang.org/2025/06/26/Rust-1.88.0/#:~:text=proc_macro%3A%3ASpan%3A%3Aline) due to `$line` meta variable line retrieval via `proc_macro::Span::line`.

## Method inserted by default

If you don't specify anything for `#[hooq]`, the following method is inserted by default:

```ignore
.inspect_err(|e| {
    let path = $path;
    let line = $line;
    let col = $col;
    let expr = ::hooq::summary!($source);

    ::std::eprintln!("[{path}:{line}:{col}] {e:?}\n{expr}");
})
```

You can switch the method to hook with the inert attribute `#[hooq::method(...)]`. Also, when you specify a flavor at the macro call site such as `#[hooq(log)]` or `#[hooq(anyhow)]`, the inserted method will change according to that flavor!

## Attributes Quick Reference

The hooq macro can modify its behavior using inert attributes such as `#[hooq::method(...)]`.

See mdbook's [Attributes](https://anotherhollow1125.github.io/hooq/latest/en/reference/attributes.html) for more details!

| Name | Type | Description |
|:----|:----|:----|
| flavor | Macro root meta | Apply settings for the specified flavor |
| trait_use | Macro root meta | Insert `use XXX as _;` before the item for the specified path (`XXX`) |
| method | Inert attribute | Set the method to insert (or expression to replace in replace mode) |
| skip_all / skip | Inert attribute | Disable hooking for expressions with this attribute |
| hook_targets | Inert attribute | Toggle hooking for `?`, `return`, and tail expressions (default: all three) |
| tail_expr_idents | Inert attribute | Specify idents to hook when they appear in tail position (default: `Err`) |
| ignore_tail_expr_idents | Inert attribute | Specify idents to not hook even when they would be hooked (default: `Ok`) |
| result_types | Inert attribute | Specify return types for functions to hook `return` and tail expressions (default: `Result`) |
| hook_in_macros | Inert attribute | Specify whether to hook inside macros (default: `true`) |
| binding | Inert attribute | Create meta variables that are replaced with specified literals/expressions |

Usage example:

```rust
use hooq::hooq;

mod sub {
    pub trait Trait {}
}

fn failable<T>(val: T) -> Result<T, String> {
    Ok(val)
}

#[hooq(flavor = "hook", trait_use(sub::Trait))] // Attribute macro root.
#[hooq::method(.inspect_err(|_| { let _ = "error!"; }))] // All following attributes are inert.
#[hooq::hook_targets("?", "return", "tail_expr")]
#[hooq::tail_expr_idents("Err")]
#[hooq::ignore_tail_expr_idents("Ok")]
#[hooq::result_types("Result")]
#[hooq::hook_in_macros(true)]
#[hooq::binding(xxx = "xxx_value")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    failable(())?;

    #[hooq::skip_all]
    if failable(false)? {
        failable(())?;
    }

    #[hooq::skip]
    if failable(false)? {
        // Next line is not skipped.
        failable(())?;
    }

    #[hooq::method(.inspect_err(|_| { let _ = $xxx; }))]
    failable(())?;

    Ok(())
}
```

## Meta Variables Quick Reference

With method settings for insertion such as `#[hooq::method(...)]`, you can use information convenient for debugging and logging through meta variables such as `$line`.

See mdbook's [Meta Variables](https://anotherhollow1125.github.io/hooq/latest/en/reference/meta_vars.html) for more details!

| Name | Literal Type | Description |
|:----|:-----------|:----|
| `$line` | usize integer | Line number where the hook target is located |
| `$column` or `$col` | usize integer | Column number where the hook target is located |
| `$path` | string | Relative path to the file containing the hook target |
| `$file` | string | Name of the file containing the hook target |
| `$source` | expression | Expression for insertion/replacement target used for debugging/logging (note the difference from `$expr`) |
| `$count` or `$nth` | string | Indicates which replacement target this is |
| `$fn_name` or `$fnname` | string | Name of the function containing the hook target |
| `$fn_sig` or `$fnsig` | string | Signature of the function containing the hook target |
| `$xxx` (example) | (arbitrary) | User-defined meta variable via inert attribute `#[hooq::xxx = ...]` |
| `$bindings` or `$vars` | [`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html) | All meta variable bindings |
| `$hooq_meta` or `$hooqmeta` | [`hooq::HooqMeta`](https://docs.rs/hooq/latest/hooq/struct.HooqMeta.html) | Struct combining `$line`, `$col`, `$path`, `$file`, `$source`, `$count`, and `$bindings` |
| `$expr` | expression | Expression for replacement target used for replacement (note the difference from `$source`) |
| `$so_far` or `$sofar` | expression | Hook set so far, mainly used for insertion |

Usage example:

```rust
use hooq::hooq;

fn failable<T>(val: T) -> Result<T, String> {
    Ok(val)
}

#[hooq]
#[hooq::xxx = "user defined binding."]
#[hooq::method(.inspect_err(|_| {
    // Fundamental information provided by hooq.
    let _line = $line;
    let _column = $column;
    let _path = $path;
    let _file = $file;
    let _source = stringify!($source);
    let _count = $count;
    let _fn_name = $fn_name;
    let _fn_sig = $fn_sig;

    // Meta vars defined by user.
    let _xxx = $xxx;
    let _bindings = $bindings;

    // All information summarized up to this point.
    let _hooq_meta = $hooq_meta;
}))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    failable(())?;

    Ok(())
}
```

## Built-in Flavors Quick Reference

Configuring settings with attributes every time is tedious. hooq has a feature called "flavors" that allows you to pre-configure settings.

Flavors can be user-defined, and hooq also provides several built-in ones!

See mdbook's [Flavors](https://anotherhollow1125.github.io/hooq/latest/en/reference/flavors.html) for more details! Below are the pre-configured flavors (built-in flavors).

| Flavor Name | Feature | Description |
|:----------|:--------|:----|
| default | - | The flavor used when nothing is specified. Can be overridden with hooq.toml |
| empty | - | A flavor for when you don't want to hook anything. Cannot be overridden |
| hook | - | Inserts a `hook` method taking [`hooq::HooqMeta`](https://docs.rs/hooq/latest/hooq/struct.HooqMeta.html) as an argument. Intended for use via user-defined traits. Can be overridden |
| anyhow | anyhow | Inserts the [`with_context`](https://docs.rs/anyhow/latest/anyhow/trait.Context.html#tymethod.with_context) method. Can be overridden |
| eyre / color_eyre | eyre | Inserts the [`wrap_err_with`](https://docs.rs/eyre/latest/eyre/trait.WrapErr.html#tymethod.wrap_err_with) method. Can be overridden |
| log | log | Inserts an `inspect_err` method calling [`::log::error!`](https://docs.rs/log/latest/log/macro.error.html). Can be overridden |
| tracing | tracing | Inserts an `inspect_err` method calling [`::tracing::error!`](https://docs.rs/tracing/latest/tracing/macro.error.html). Can be overridden |

Usage example:

```rust,should_panic
use hooq::hooq;

#[hooq(anyhow)]
fn func1() -> anyhow::Result<i32> {
    Err(anyhow::anyhow!("Error in func1"))
}

#[hooq(anyhow)]
fn func2() -> anyhow::Result<i32> {
    func1()
}

#[hooq(anyhow)]
fn main() -> anyhow::Result<()> {
    func2()?;

    Ok(())
}
```

Output example:

```plaintext
Error: [mdbook-source-code/flavor-anyhow/src/main.rs:15:12]
  15>    func2()?
    |

Caused by:
    0: [mdbook-source-code/flavor-anyhow/src/main.rs:10:5]
         10>    func1()
           |
    1: [mdbook-source-code/flavor-anyhow/src/main.rs:5:5]
          5>    Err(anyhow::anyhow!("Error in func1"))
           |
    2: Error in func1
```

## License

This project is dual-licensed:

- Apache License, Version 2.0 ([LICENSE-APACHE](https://github.com/anotherhollow1125/hooq/blob/main/LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](https://github.com/anotherhollow1125/hooq/blob/main/LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

## Contributing

Issues and PRs are welcome!

Contribution guidelines are summarized on a separate page with the following topics:

- Snapshot testing
- CI
- [`sync.rs`](https://github.com/anotherhollow1125/hooq/blob/main/.github/scripts/sync.rs) command
- Languages (English/Japanese)
- Stance on using generative AI

[CONTRIBUTING](https://github.com/anotherhollow1125/hooq/blob/main/docs/CONTRIBUTING.md)
