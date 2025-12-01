# Turn `?` into `.unwrap()` Behind a Feature

As explained in [Method](../reference/method.md#removing--via-trailing-), with the `consume-question` feature enabled, appending `!` to the method allows consuming `?`.

We can use this to treat `?` as an alias for `.unwrap()`.

Example: when an `unwrap` feature is enabled, replace `?` with `.unwrap()`. Cargo.toml features:

```toml
[dependencies]
hooq = { version = "*", features = ["consume-question"] }

# ..

{{#include ../../../../../mdbook-source-code/recipe-unwrap/Cargo.toml:13:14}}
```

Prepare a flavor in `hooq.toml`. We restrict `hook_targets` to only `?` so tails/returns are unaffected.

```toml
{{#include ../../../../../mdbook-source-code/recipe-unwrap/hooq.toml}}
```

`main.rs`:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/recipe-unwrap/src/main.rs}}
```

Without the `unwrap` feature, expansion uses the [empty](../reference/flavors.md#empty) flavor and nothing changes:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/recipe-unwrap/src/main.expanded.rs:6:20}}
```

With the `unwrap` feature, expansion replaces with `.unwrap()`:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/recipe-unwrap/src/main.unwrap.expanded.rs:6:20}}
```

Highlights:
- You can replace `?` with special behaviors like `.unwrap()`.
- Feature‑gated `#[cfg_attr(..., hooq(...))]` lets you vary hooks by build configuration.
