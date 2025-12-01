# Batch Apply Under a Module

hooq can be attached not only to functions but to items containing executable syntax.

Here, an "item" refers to Rust syntax elements that can appear directly inside a module/scope: functions, `mod`, `impl` blocks, trait blocks, etc. (See [`syn::Item`](https://docs.rs/syn/latest/syn/enum.Item.html).) Items without executable syntax are irrelevant.

When a `mod { ... }` (brace form, not `mod xxx;`) is annotated with `#[hooq]`, inner functions can be hooked without annotating each function.

This recursive behavior is not limited to `mod`. If functions/closures nest, hooq will greedily hook according to settings for inner functions/closures as well. Use `#[hooq::skip_all]` if compilation fails or you want to disable in a region.

Puzzle example below illustrating which inner values get hooked:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/recipe-batch-apply/src/main.rs}}
```

Expansion shows aggressive yet fairly natural hooking for a macro[^macro]:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/recipe-batch-apply/src/main.expanded.rs:7:38}}
```

[^macro]: Procedural macros lack rich reflection; we cannot type‑infer `Result`.
