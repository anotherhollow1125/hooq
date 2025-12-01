# Using in Functions Returning Option

We previously introduced [controlling hook target decisions via attributes](../reference/attributes.md#control-hook-targets), which may feel abstract. Here is a more concrete scenario: target only `Option` values.

In the example below, we hook functions defined inside `main` (nesting is supported; see [Batch apply](./batch_apply.md)). We want to inspect when a value is `Some`.

- `tail_expr_idents`: set to `Some` to hook only `Some`.
- `ignore_tail_expr_idents`: set to `None` to avoid wasteful hooks.
- `result_types`: set to `Option` so tail/return values are considered.

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/recipe-option/src/main.rs}}
```

Expansion:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/recipe-option/src/main.expanded.rs:8:27}}
```

The point: while `Result`/`Ok`/`Err` are common defaults, hooq does not hard‑code these; you can target other types/idents.

Potentially useful once [`Try` trait](https://doc.rust-lang.org/std/ops/trait.Try.html) stabilizes.
