# Perfect color-eyre

[`color-eyre`](https://docs.rs/color-eyre/latest/color_eyre/) provides advanced ways to obtain BACKTRACE and SPANTRACE and is hard to ignore.

Let’s extend the [official example](https://docs.rs/color-eyre/latest/color_eyre/#:~:text=%F0%9F%A6%80%20v1.44.0%0A%E2%9D%AF-,RUST_LIB_BACKTRACE%3D1%20cargo%20run%20%2D%2Dexample%20usage,-Finished%20dev%20%5Bunoptimized) by adding hooq’s pseudo trace.

Create a new project and set the following in Cargo.toml (replace `hooq` with the latest version):

```toml
{{#include ../../../../../mdbook-source-code/recipe-color-eyre/Cargo.toml:13:22}}
```

Write `main.rs`. `use hooq::hooq;` and annotate `#[hooq(color_eyre)]` above `#[instrument]` (we also add `.without_time()` to stabilize snapshots).

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/recipe-color-eyre/src/main.rs}}
```

Run with `RUST_LIB_BACKTRACE=full` to also see the BACKTRACE:

```rust
{{#include ../../../../../mdbook-source-code/recipe-color-eyre/tests/snapshots/test__recipe-color-eyre.snap:9:}}
```

This is likely the most detailed output.
