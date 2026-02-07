# Flavors

Flavors are presets that bundle hooq settings. Built‑in flavors:

| Name | feature | Contents |
|:-----|:--------|:---------|
| [default](#default) | - | Default when nothing is specified; overridable via `hooq.toml`. |
| [empty](#empty) | - | Disables hooking; inert attributes still processed. Not overridable. |
| [hook](#hook) | - | Inserts a `hook` method taking [`hooq::HooqMeta`](https://docs.rs/hooq/latest/hooq/struct.HooqMeta.html); designed for user traits. Overridable. |
| [anyhow](#anyhow) | anyhow | Inserts `.with_context(...)`. Overridable. |
| [eyre](#eyre--color_eyre) / [color_eyre](#eyre--color_eyre) | eyre | Inserts `.wrap_err_with(...)`. Overridable. |
| [log](#log) | log | Inserts `inspect_err` that calls [`::log::log!`](https://docs.rs/log/latest/log/macro.log.html). Overridable. |
| [tracing](#tracing) | tracing | Inserts `inspect_err` that calls [`::tracing::event!`](https://docs.rs/tracing/latest/tracing/macro.event.html). Overridable. |

Flavor features are part of the default feature set, so you usually do not need to enable them explicitly.

Users can define flavors in a `hooq.toml` at crate root.

## User‑Defined Flavors

`hooq.toml` uses table names as flavor names with fields:

| Field | Type | Description |
|:------|:-----|:------------|
| trait_uses | array of strings | Trait paths to import. |
| method | string | Method/expression to insert/replace. |
| hook_targets | array of strings | Any of `"?"`, `"return"`, `"tail_expr"`. |
| tail_expr_idents | array of strings | Idents like `"Err"`. |
| ignore_tail_expr_idents | array of strings | Idents like `"Ok"`. |
| result_types | array of strings | Return type idents like `"Result"`. |
| hook_in_macros | bool | `true` or `false`. |
| bindings | inline table | Arbitrary bindings; note string literals must be quoted with `\"`. |

All built‑in (except `empty`) can be overridden by defining the same table name. Sub‑tables other than `bindings` are sub‑flavors and inherit from their parent.

See [Attributes](./attributes.md) for how to apply flavors.

Example `hooq.toml`:

```toml
{{#include ../../../../../mdbook-source-code/my-flavor/hooq.toml}}
```

Usage:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/my-flavor/src/main.rs}}
```

Expansion:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/my-flavor/src/main.expanded.rs:13:25}}
```

## default

Default configuration when using `#[hooq]`.

configured as follows. (To keep the documentation consistent, this is excerpted directly from the source code; the same applies below.)

```rust
{{#rustdoc_include ../../../../../hooq-macros/src/impls/flavor/mod.rs:91:108}}
```

Default method (Please ignore Japanese comments):

```rust
{{#rustdoc_include ../../../../../hooq-macros/src/impls/flavor/mod.rs:110:129}}
```

Usage:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/flavor-default/src/main.rs}}
```

Result:

```bash
{{#include ../../../../../mdbook-source-code/flavor-default/tests/snapshots/test__flavor-default.snap:8:11}}
```

You can override via `hooq.toml`.

## empty

Disables hooking; intended for conditional builds like `#[cfg_attr(feature = "...", hooq(empty))]`.

configured as follows.

```rust
{{#rustdoc_include ../../../../../hooq-macros/src/impls/flavor/presets/empty.rs:10:36}}
```

Not overridable.

## hook

configured as follows. (Please ignore Japanese comments)

```rust
{{#rustdoc_include ../../../../../hooq-macros/src/impls/flavor/presets/hook.rs:9:29}}
```

Designed for user traits to implement a `hook` method. It is useful when you do not want to use `hooq.toml`.

Usage:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/flavor-hook/src/main.rs}}
```

The second argument is a closure (`meta_fn`) for lazy evaluation to avoid constructing `HooqMeta` everywhere.

Expansion:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/flavor-hook/src/main.expanded.rs:37:53}}
```

## anyhow

> Requires `anyhow` feature (included in default).

This flavor is intended to be used with the [anyhow crate](https://docs.rs/anyhow/latest/anyhow/).

configured as follows.

```rust
{{#rustdoc_include ../../../../../hooq-macros/src/impls/flavor/presets/anyhow.rs:9:37}}
```

Imports `anyhow::Context` for [`.with_context(...)`](https://docs.rs/anyhow/latest/anyhow/trait.Context.html#tymethod.with_context).

Usage:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/flavor-anyhow/src/main.rs}}
```

Result:

```bash
{{#include ../../../../../mdbook-source-code/flavor-anyhow/tests/snapshots/test__flavor-anyhow.snap:8:19}}
```

## eyre / color_eyre

> Requires `eyre` feature (included in default).

This flavor is intended to be used with the [eyre crate](https://docs.rs/eyre/latest/eyre/).

configured as follows.

```rust
{{#rustdoc_include ../../../../../hooq-macros/src/impls/flavor/presets/eyre.rs:9:54}}
```

Imports `eyre::WrapErr` and `eyre::ContextCompat` for [`.wrap_err_with(...)`](https://docs.rs/eyre/latest/eyre/trait.WrapErr.html#tymethod.wrap_err_with).

Usage:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/flavor-eyre/src/main.rs}}
```

Result:

```bash
{{#include ../../../../../mdbook-source-code/flavor-eyre/tests/snapshots/test__flavor-eyre.snap:8:22}}
```

A `color_eyre` flavor is also provided that inserts `use ::color_eyre::eyre::WrapErr as _;` instead of `use ::eyre::WrapErr as _;`.

## log

> Requires `log` feature (included in default).

This flavor is intended to be used with the [log crate](https://docs.rs/log/latest/log/).

configured as follows.

```rust
{{#rustdoc_include ../../../../../hooq-macros/src/impls/flavor/presets/log.rs:11:67}}
```

By default, logs are emitted at [`log::Level::Error`](https://docs.rs/log/latest/log/enum.Level.html#variant.Error).

You can change the log level either by using a sub-flavor like `#[hooq(log::warn)]`, or by binding a [`log::Level`](https://docs.rs/log/latest/log/enum.Level.html) variant to the `$level` meta variable.

Usage:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/flavor-log/src/main.rs}}
```

Result:

```bash
{{#include ../../../../../mdbook-source-code/flavor-log/tests/snapshots/test__flavor-log.snap:8:27}}
```

## tracing

> Requires `tracing` feature (included in default).

This flavor is intended to be used with the [tracing crate](https://docs.rs/tracing/latest/tracing/).

configured as follows.

```rust
{{#rustdoc_include ../../../../../hooq-macros/src/impls/flavor/presets/tracing.rs:11:74}}
```

By default, logs are emitted at [`tracing::Level::ERROR`](https://docs.rs/tracing/latest/tracing/struct.Level.html#associatedconstant.ERROR).

You can change the level either by using a sub-flavor like `#[hooq(tracing::warn)]`, or by binding a [`tracing::Level`](https://docs.rs/tracing/latest/tracing/struct.Level.html) value to the `$level` meta variable.

Place `#[hooq(tracing)]` above `#[tracing::instrument]` to ensure order.

Usage:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/flavor-tracing/src/main.rs}}
```

Result:

```bash
{{#include ../../../../../mdbook-source-code/flavor-tracing/tests/snapshots/test__flavor-tracing.snap:5:12}}
```
