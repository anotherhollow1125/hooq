# Introduction

[hooq](https://crates.io/crates/hooq) is an attribute macro that can (when desired) insert methods in front of the `?` operator[^try], `return`, and tail expressions.

[^try]: The former `try` macro.

```rust
{{#rustdoc_include ../../../../mdbook-source-code/eye_catch/src/main.rs::18}}
```

It ships with rich presets so you can easily hook error logging and similar behavior onto `Result` values.

## Why use hooq?

We will motivate hooq using the following source code containing a function that returns a `Result` type:

```rust
{{#rustdoc_include ../../../../mdbook-source-code/index_without_hooq/src/main.rs::24}}
```

If you run it with all required environment variables set, it works without errors:

```bash
$ APP_HOST=127.0.0.1 APP_PORT=10 cargo run -q
{{#include ../../../../mdbook-source-code/index_without_hooq/tests/snapshots/test__index_without_hooq_with_envs.snap:6:6}}
```

Now run it in a way that triggers an error:

```bash
$ APP_PORT=10 cargo run -q
{{#include ../../../../mdbook-source-code/index_without_hooq/tests/snapshots/test__index_without_hooq_with_app_port.snap:8:8}}
```

The contents of the `Box<dyn Error>` returned from `main` are printed; we (probably) have a missing environment variable.

But this error output is terrible:

- You cannot tell the context or what kind of error it is
- **You cannot tell where the error occurred**

Most likely the author of this Rust program did not want to build a formal application with fine‑grained error handling, but just a small casual CLI tool. Rust’s error output, however, can be rather unforgiving if you cut corners[^unwrap].

[^unwrap]: "Couldn’t we just use `unwrap` at this scale?" I hate perceptive brats like you… Even so, not being able to use `Result` comfortably would be inconvenient, so the hooq macro remains meaningful.

**Enter the `hooq` attribute macro!**

```rust
{{#rustdoc_include ../../../../mdbook-source-code/index/src/main.rs::28}}
```

Simply adding `#[hooq]` magically produces a pseudo stack trace for errors:

```bash
$ APP_PORT=10 cargo run -q
{{#include ../../../../mdbook-source-code/index/tests/snapshots/test__index_with_app_port.snap:8:14}}
```

Apparently the `APP_HOST` environment variable was missing. We can also see how the error propagated (line 8, then line 21, etc.).

What exactly did the `hooq` macro do? That will be [explained later](./tutorial/index.md).
