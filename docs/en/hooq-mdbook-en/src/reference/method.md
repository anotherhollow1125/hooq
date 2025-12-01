# Method

Specify hooked methods via `#[hooq::method(...)]` or by flavors. The `...` has two modes:

- If it starts with `.`, it is in **insertion mode**, inserting the method between the expression and `?` (or at the end for `return`/tail).
- Otherwise it is in **replacement mode**, replacing the expression. Use `$expr` to reference the original expression (already recursively hooked where needed).

Meta variables (see [Meta Variables](./meta_vars.md)) can be used in these expressions.

## Insertion Mode

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/method_insert_mode/src/main.rs}}
```

You can chain with `.$so_far` to prepend/append relative to the existing chain.

Expansion:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/method_insert_mode/src/main.expanded.rs:10:30}}
```

## Replacement Mode

If it does not start with `.` (dot), it is treated as a replacement function, replacing the original expression with the configured expression. The expression being replaced can be accessed using the `$expr` meta variable. (While it's possible to write without using `$expr`, it would likely not be useful.)

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/method_replace_mode/src/main.rs}}
```

Expansion:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/method_replace_mode/src/main.expanded.rs:21:24}}
```

<div class="warning">

Use `$expr` (not `$source`) in replacement mode. `$expr` holds the expression after inner hooks have been applied, while `$source` is for logging/display of the original tokens.
</div>

## Removing `?` via Trailing `!`

When hooking the `?` operator, appending `!` to the method consumes the trailing `?`.

> Requires the [`consume-question`](./features.md) feature.

```bash
cargo add hooq --features consume-question
```

Example aliasing `?` to `.unwrap()`:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/consume_question/src/main.rs}}
```

Expansion:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/consume_question/src/main.expanded.rs:6:12}}
```
