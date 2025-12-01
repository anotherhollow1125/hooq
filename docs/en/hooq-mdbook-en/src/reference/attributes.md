# Attributes

The behavior of the hooq macro is controlled by the root meta on `#[hooq(...)]` and by inert attributes inserted afterwards (`#[hooq::attribute(...)]`). This page explains configurable items and defaults.

**Quick Reference**

| Name | Kind | Description |
|:-----|:-----|:------------|
| [flavor](#flavor) | root meta | Apply settings from a named flavor. |
| [trait_use](#trait_use) | root meta | Insert `use XXX as _;` before the item for the given trait path(s). |
| [method](#method) | inert | Configure the method to insert/replace. |
| [skip_all](#skip_all) / [skip](#skip) | inert | Skip hooking the annotated expression; `skip_all` also skips inside it. |
| [hook_targets](#hook_targets) | inert | Enable hooking on `?`, `return`, and/or `tail_expr` (default: all). |
| [tail_expr_idents](#tail_expr_idents) | inert | Idents to hook when appearing as tail/return values (default: `Err`). |
| [ignore_tail_expr_idents](#ignore_tail_expr_idents) | inert | Idents to avoid hooking even if otherwise eligible (default: `Ok`). |
| [result_types](#result_types) | inert | Function return type idents considered hook targets for `return`/tail (default: `Result`). |
| [hook_in_macros](#hook_in_macros) | inert | Whether to hook inside macro invocations (default: `true`). |
| [binding](#binding) | inert | Define custom meta variable bindings (expression or literal).

- Root meta: attributes written inside `#[hooq(...)]`.
- Inert attributes: `#[hooq::attribute(...)]` placed in various positions after the macro.

Example with all attributes:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/all_attr/src/main.rs}}
```

Expansion:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/all_attr/src/main.expanded.rs}}
```

## Root Meta

Two fields are available inside `#[hooq(...)]`:

| Name | Syntax |
|:-----|:-------|
| flavor | `#[hooq(FLAVOR_NAME)]` or `#[hooq(flavor = "FLAVOR_NAME")]` |
| trait_use | `#[hooq(trait_use(PATH, ...))]` or `#[hooq(trait_uses(PATH, ...))]` |

### flavor

Apply base settings from a flavor:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/attribute-flavor-1/src/main.rs}}
```

`hooq.toml` example:

```toml
{{#include ../../../../../mdbook-source-code/attribute-flavor-1/hooq.toml}}
```

Expansion:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/attribute-flavor-1/src/main.expanded.rs:6:}}
```

Sub‑flavors inherit and override parent flavor settings. Use dotted or path‑like names (e.g. `base.sub` or `base::sub`), and they can nest.

Further info, especially built‑ins like `anyhow`, writing `hooq.toml`, etc.: see [Flavors](./flavors.md).

### trait_use

Insert `#[allow(unused)] use PATH as _;` above the item:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/trait_use/src/main.rs}}
```

Expansion excerpt:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/trait_use/src/main.expanded.rs:20:25}}
```

Useful when the hooked method requires a trait (e.g. `anyhow::Context` for `.with_context(...)`). You can also configure `trait_uses` in `hooq.toml` so a flavor brings the necessary imports.

## method

| Name | Syntax |
|:-----|:-------|
| method | `#[hooq::method(...)]` |

Set the method via inert attribute. Works both immediately under `#[hooq]` and inside the function.

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/attribute-method/src/main.rs}}
```

Expansion:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/attribute-method/src/main.expanded.rs:6:17}}
```

Two modes exist:
- Insertion mode: starts with `.`; inserts between the expression and `?` (or at the end for `return`/tail).
- Replacement mode: otherwise; replace the target using an expression. Use `$expr` to access the original (already recursively hooked) expression.

Meta variables like `$line`, `$source`, `$fn_name` are available. Default flavor inserts:

```rust
.inspect_err(|e| {
    let path = $path;
    let line = $line;
    let col = $col;
    let expr = ::hooq::summary!($source);

    ::std::eprintln!("[{path}:{line}:{col}] {e:?}\n{expr}");
})
```

See [Method](./method.md) and [Meta Variables](./meta_vars.md).

## Skipping Hooks

| Name | Syntax |
|:-----|:-------|
| skip_all | `#[hooq::skip_all]` |
| skip | `#[hooq::skip]` |

### skip_all

Prevent hooks inside the annotated expression:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/skip_all/src/main.rs}}
```

Expansion:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/skip_all/src/main.expanded.rs:7:15}}
```

### skip

Skip only within the parent scope (children can still be hooked). Helpful when nested tail expressions would produce noisy logs.

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/skip/src/main.rs::24}}
```

Expansion:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/skip/src/main.expanded.rs:7:19}}
```

## Control Hook Targets

Inert attributes to tweak whether/how hooks apply.

| Name | Syntax | Values |
|:-----|:-------|:-------|
| hook_targets | `#[hooq::hook_targets(...)]` | Any of `"?"`, `"return"`, `"tail_expr"`. |
| tail_expr_idents | `#[hooq::tail_expr_idents(...)]` | Idents like `Err`. |
| ignore_tail_expr_idents | `#[hooq::ignore_tail_expr_idents(...)]` | Idents like `Ok`. |
| result_types | `#[hooq::result_types(...)]` | Return type idents like `Result`. |
| hook_in_macros | `#[hooq::hook_in_macros(...)]` | `true` or `false`.

Priority rules:
- If `skip_all` is present, do not hook (for `skip`, skip only in the same scope excluding children).
- If target is inside a macro call (like `println!(...)` ) and `hook_in_macros` is `false`, do not hook.
- For `?`: hook if included in `hook_targets`.
- For `return`: hook if included in `hook_targets`, and either ident is in `tail_expr_idents` or return type ident is in `result_types` and ident is not in `ignore_tail_expr_idents`.
- For tail: similar rules as `return`.

### hook_targets

You can specify whether to hook each of the following three types: the `?` operator (Question Operator), `return`, and tail expressions (`tail_expr`). By default, all three types are hooked.

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/hook_targets/src/main.rs::56}}
```

Expansion:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/hook_targets/src/main.expanded.rs:10:31}}
```

### tail_expr_idents

Idents to hook regardless of `result_types`. Default: `Err`.

<div class="warning">

Identifiers must be single idents (e.g., `Zzz`), not paths like `xxx::yyy::Zzz`.

Matching uses the last segment of a path (`xxx::yyy::Zzz` matches by `Zzz`).
</div>


```rust
{{#rustdoc_include ../../../../../mdbook-source-code/tail_expr_idents/src/main.rs}}
```

Expansion:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/tail_expr_idents/src/main.expanded.rs:10:26}}
```

### ignore_tail_expr_idents

Avoid hooking these idents even when otherwise eligible. Default: `Ok`.

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/ignore_tail_expr_idents/src/main.rs}}
```

<div class="warning">

As shown above, you can also achieve the same behavior without `ignore_tail_expr_idents` by prefixing an ident with `!` in `tail_expr_idents` (e.g., `!Ok`).
</div>

Expansion:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/ignore_tail_expr_idents/src/main.expanded.rs:10:17}}
```

<div class="warning">

If the same ident appears in both lists, it will be hooked due to simple mechanics; prefer using `!Ident` via `tail_expr_idents` instead.
</div>

### result_types

Function return type idents considered for `return`/tail hooks. Default: `Result`.

<div class="warning">

As with `tail_expr_idents`, identifiers must be single idents (e.g., `Zzz`), not full paths like `xxx::yyy::Zzz`.

Matching uses the final segment of a path (`xxx::yyy::Zzz` matches by `Zzz`).
</div>

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/result_types/src/main.rs::39}}
```

Expansion:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/result_types/src/main.expanded.rs:10:23}}
```

### hook_in_macros

Control hooking inside function‑like macros.

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/hook_in_macros/src/main.rs}}
```

Expansion:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/hook_in_macros/src/main.expanded.rs:10:18}}
```

The arguments to function-like macros do not always follow standard Rust syntax, and hooking them incurs a small parsing cost. This inert attribute allows you to disable hooking inside macros. If you don't need to hook inside macros, setting this to `false` may slightly reduce compile times.

## binding

Define user [meta variables](./meta_vars.md) (bindings). Several forms:

| Syntax | Note |
|:-------|:-----|
| `#[hooq::binding(xxx = ...)]` | |
| `#[hooq::var(xxx = ...)]` | |
| `#[hooq::xxx = ...]` | `xxx` must not conflict with other inert attribute names. |

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/attribute-binding/src/main.rs}}
```

Expansion:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/attribute-binding/src/main.expanded.rs:10:34}}
```

See also [Meta Variables — Bindings](./meta_vars.md#user-defined-meta-variable-bindings).

## Partial Application via Flavor

You can partially apply flavor settings via inert attributes:

- `#[hooq::attribute = flavor_name]` for a single field.
- `#[hooq::flavor = flavor_name]` to override all inert‑attribute fields.
- `#[hooq::bindings = flavor_name]` to override existing user bindings.

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/attribute-part-flavor/src/main.rs}}
```

`hooq.toml`:

```toml
{{#include ../../../../../mdbook-source-code/attribute-part-flavor/hooq.toml}}
```

Expansion:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/attribute-part-flavor/src/main.expanded.rs:6:43}}
```
