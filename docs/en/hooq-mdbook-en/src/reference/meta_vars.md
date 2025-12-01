# Meta Variables

Inside `#[hooq::method(...)]`, you can use special **meta variables** (prefixed with `$`) that are replaced with values.

**Quick Reference**

| Name | Literal Kind | Description |
|:-----|:-----|:------------|
| [`$line`](#line) | usize | Line number of the target. |
| [`$column`](#column) or [`$col`](#column) | usize | Column number of the target. |
| [`$path`](#path) | string | Relative path to the file of the target. |
| [`$file`](#file) | string | File name of the target. |
| [`$source`](#source) | expression | Original tokens of the target (for logging; differs from [`$expr`](#expr)). |
| [`$count`](#count) or [`$nth`](#count) | string | Display which numbered target within the function. |
| [`$fn_name`](#fn_name) or [`$fnname`](#fn_name) | string | Name of the function containing the target. |
| [`$fn_sig`](#fn_sig) or [`$fnsig`](#fn_sig) | string | Signature of the function/closure containing the target. |
| `$xxx` | (any) | User‑defined meta variable via inert attribute. |
| [`$bindings`](#bindings) or [`$vars`](#bindings) | [`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html) | All user bindings. |
| [`$hooq_meta`](#hooq_meta) or [`$hooqmeta`](#hooq_meta) | [`hooq::HooqMeta`](https://docs.rs/hooq/latest/hooq/struct.HooqMeta.html) | Struct bundling key meta information. |
| [`$expr`](#expr) | expression | Original expression for replacement mode (after inner hooks). |
| [`$so_far`](#so_far) or [`$sofar`](#so_far) | expression | Current insertion chain, used to chain further.

Example using most variables:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/meta-vars-all/src/main.rs}}
```

Expansion:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/meta-vars-all/src/main.expanded.rs:10:66}}
```

## Target Information

### line

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/meta-vars-line/src/main.rs}}
```

Expansion:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/meta-vars-line/src/main.expanded.rs:10:16}}
```

<div class="warning">

Prefer `$line` over `line!()`; the latter points to the attribute location, not the actual target.
</div>

### column

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/meta-vars-column/src/main.rs}}
```

Expansion:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/meta-vars-column/src/main.expanded.rs:10:16}}
```

### path

Replaced with the relative path from the crate root (`CARGO_MANIFEST_DIR`) to the file containing the hook target.

However, the relative path's starting point may not be the crate root in cases such as when using workspaces. Since procedural macros are designed not to obtain accurate absolute file paths, hooq does not provide meta variables for retrieving absolute paths.

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/meta-vars-path/src/main.rs}}
```

Expansion:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/meta-vars-path/src/main.expanded.rs:10:16}}
```

### file

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/meta-vars-file/src/main.rs}}
```

Expansion:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/meta-vars-file/src/main.expanded.rs:10:16}}
```

### source

This meta variable provides the token stream (expression) of the hook target before any hooks are applied by the hooq macro. It is intended for debugging purposes.

<div class="warning">

In contrast, `$expr` represents the expression after hooks have already been applied internally. `$expr` is used in replacement mode to determine where to embed the original expression.
</div>

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/meta-vars-source/src/main.rs}}
```

Expansion:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/meta-vars-source/src/main.expanded.rs:10:16}}
```

Stringification helpers commonly used with `$source`:
- [`stringify!` macro](https://doc.rust-lang.org/std/macro.stringify.html)
- [`hooq::summary!` macro](https://docs.rs/hooq/latest/hooq/macro.summary.html)
- Macros under [`hooq::source_excerpt_helpers`](https://docs.rs/hooq/latest/hooq/source_excerpt_helpers/index.html)

Example with `summary!`:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/meta-vars-source-summary/src/main.rs}}
```

Result excerpt:

```bash
{{#include ../../../../../mdbook-source-code/meta-vars-source-summary/tests/snapshots/test__meta-vars-source-summary.snap:8:14}}
```

### count

Indicates which numbered target this is within the function for each target type (`?`, `return`, or tail expression).

This meta variable is a remnant from when `$line` could only be obtained on nightly. In most cases, `$line` is more straightforward.

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/meta-vars-count/src/main.rs}}
```

Expansion:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/meta-vars-count/src/main.expanded.rs:10:16}}
```

### fn_name

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/meta-vars-fn_name/src/main.rs}}
```

Expansion:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/meta-vars-fn_name/src/main.expanded.rs:10:26}}
```

### fn_sig

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/meta-vars-fn_sig/src/main.rs}}
```

Expansion:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/meta-vars-fn_sig/src/main.expanded.rs:10:26}}
```

## User‑Defined Meta Variables (Bindings)

Define via inert attributes `#[hooq::xxx = ...]` or in `hooq.toml` under `bindings`.

For details on how to define them, see the respective pages:

- [binding (inert attribute)](./attributes.md#binding)
- [Flavors](./flavors.md#userdefined-flavors)

Example (inert attribute):

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/meta-vars-binding/src/main.rs}}
```

Expansion:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/meta-vars-binding/src/main.expanded.rs:14:28}}
```

### bindings

Retrieve all user-defined meta variables (bindings) as a [`HashMap<String, BindingPayload>`](https://doc.rust-lang.org/std/collections/struct.HashMap.html).

[`BindingPayload`](https://docs.rs/hooq/latest/hooq/struct.BindingPayload.html) stores the stringified binding expression and a value using `Rc<dyn Any>`.

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/meta-vars-bindings/src/main.rs}}
```

Expansion:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/meta-vars-bindings/src/main.expanded.rs:14:63}}
```

### hooq_meta

Bundle meta info into `hooq::HooqMeta`.

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/meta-vars-hooq_meta/src/main.rs}}
```

Expansion:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/meta-vars-hooq_meta/src/main.expanded.rs:10:38}}
```

See also the [`hook` flavor](./flavors.md#hook).

## Advanced Meta Variables for Hook Construction

The meta variables introduced so far are primarily for obtaining meta information for logging and debugging.

The remaining two, `$expr` and `$so_far`, are meta variables that assist in making methods.

### expr

Use in [replacement mode](./method.md#replacement-mode) to access the (already hooked internally) expression.

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/method_replace_mode/src/main.rs}}
```

Expansion:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/method_replace_mode/src/main.expanded.rs:21:24}}
```

### so_far

Represents the chain of [insertion‑mode](./method.md#insertion-mode) hooks so far. Drop the leading dot when stored; write as `.$so_far` when inserting.

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/method_insert_mode/src/main.rs}}
```

Expansion:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/method_insert_mode/src/main.expanded.rs:10:30}}
```
