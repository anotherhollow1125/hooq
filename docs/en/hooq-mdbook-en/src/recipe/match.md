# Reinvent `match` Desugaring

In [Turn `?` into `.unwrap()` behind a feature](./unwrap.md) we showed that with the `consume-question` feature enabled, adding `!` at the end of the method allows replacing `?`.

We can therefore replace `?` with a `match`. A fun example worth documenting.

First, enable `consume-question`:

```bash
cargo add hooq --features consume-question
```

Define the `match` expression in `hooq.toml` (the original is `$expr`). The flavor is named `my_match` since `match` is reserved:

```toml
{{#include ../../../../../mdbook-source-code/recipe-match/hooq.toml}}
```

Write `main.rs` as usual (add `#[allow(clippy::question_mark)]` to avoid lints):

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/recipe-match/src/main.rs}}
```

Expansion shows a `match` as intended:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/recipe-match/src/main.expanded.rs:6:17}}
```
