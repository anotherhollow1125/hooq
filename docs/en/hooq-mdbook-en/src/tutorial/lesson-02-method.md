# Customize Hooks with `#[hooq::method(...)]`

You can configure which method is hooked.

We will show this using an example program that prints the `name` field from the project's Cargo.toml. We add the `toml` crate first:

```bash
cargo add toml
```

Basic routine to parse a provided path as TOML:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/tutorial-1/src/main.rs:7:15}}
```

Instead of applying hooq as‑is, we will customize the hook. We also want success logging via [`inspect`](https://doc.rust-lang.org/std/result/enum.Result.html#method.inspect).

Use `#[hooq::method(...)]` to specify methods to insert. The meta variable `.$so_far` represents the chain built so far, letting us extend it with additional methods.

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/tutorial-1/src/main.rs::15}}
```

Failure still logs as before:

```bash
{{#include ../../../../../mdbook-source-code/tutorial-1/tests/snapshots/test__tutorial-1-not-exist.snap:8:11}}
```

Success now logs too:

```bash
{{#include ../../../../../mdbook-source-code/tutorial-1/tests/snapshots/test__tutorial-1.snap:6:7}}
```

Next we extract the `package.name` field with `toml::Value::get`. Because it returns an `Option`, we convert to `Result` using [`ok_or_else`](https://doc.rust-lang.org/std/option/enum.Option.html#method.ok_or_else).

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/tutorial-2/src/main.rs}}
```

As you may guess: `.ok_or_else(...)` is boilerplate. hooq can reduce it:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/tutorial-3/src/main.rs:3:17}}
```

We also added `$nth` (alias of `$count`) to show which numbered `?` caused failure. Running with an error:

```bash
{{#include ../../../../../mdbook-source-code/tutorial-3/tests/snapshots/test__tutorial-3-name-missing.snap:5:10}}
```

The TOML had `package` but not `name`; the second `?` failed as indicated.

## Skipping Hooks with `#[hooq::skip_all]`

Suppose we add a validation: “4 is ominous (like 404[^mista]), so reject names containing 4.”

[^mista]: Generally, the number `4` is considered an unlucky number, but for server-side engineers, `5` may be even more unlucky.

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/tutorial-4-compile-error/src/_main.txt:3:23}}
```

hooq also tries to hook `return Err(...)`; but `Result` lacks `ok_or_else`, causing a compile error[^error]:

[^error]: I apologies for the broken compilation error messages, I'll fix them someday.

```bash
{{#include ../../../../../mdbook-source-code/tutorial-4-compile-error/tests/snapshots/test__tutorial-4-compile-error.snap:8:21}}
```

We do not want a hook here. Add `#[hooq::skip_all]` to skip inside that scope:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/tutorial-4/src/main.rs:3:24}}
```

<details><summary>Complete program</summary>

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/tutorial-4/src/main.rs}}
```
</details>

Compilation succeeds and names containing 4 produce an error:

```bash
{{#include ../../../../../mdbook-source-code/tutorial-4/tests/snapshots/test__tutorial-4.snap:5:10}}
```

Besides `#[hooq::skip_all]`, multiple attributes exist to adjust behavior mid‑function (including attaching `#[hooq::method(...)]` to specific expressions). See [Attributes](../reference/attributes.md).

Custom methods everywhere may get tedious. The [next lesson](./lesson-03-flavor.md) introduces flavor presets.
