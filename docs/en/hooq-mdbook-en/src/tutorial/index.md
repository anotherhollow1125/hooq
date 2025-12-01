# Tutorial

This chapter introduces the basics of using hooq across three lessons plus a wrap‑up page.

1. [`#[hooq]` to capture error lines](./lesson-01-hooq.md)
2. [Customize hooks with `#[hooq::method(...)]`](./lesson-02-method.md)
3. [Create / use presets via flavors](./lesson-03-flavor.md)
4. [Summary](./conclude.md)

We assume the hooq crate has already been added.

## Adding hooq

Add via cargo:

```bash
cargo add hooq
```

Or specify the latest version in Cargo.toml (see [crates.io](https://crates.io/crates/hooq)).

Some features exist, but commonly used ones are in the default feature set. See [Features](../reference/features.md).
