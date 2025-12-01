# `#[hooq]` to Capture Error Lines

We continue with the example from [Introduction](../index.md) to explain what the hooq macro does to source code and how to use it.

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/index/src/main.rs::28}}
```

Adding the `#[hooq]` attribute macro to a function inserts the following default hooq method [`inspect_err`](https://doc.rust-lang.org/std/result/enum.Result.html#method.inspect_err) in front of each `?`, and after the value of each `return` and tail expression (only when the function's return type matches a hook target such as `Result`).

```rust
.inspect_err(|e| {
    let path = $path;
    let line = $line;
    let col = $col;
    let expr = ::hooq::summary!($source);

    ::std::eprintln!("[{path}:{line}:{col}] {e:?}\n{expr}");
})
```

Items like `$path` are hooq meta variables. See [Meta Variables](../reference/meta_vars.md) for full details; those used in the default method are:

| Meta | Literal Kind | Description |
|:-----|:-----|:------------|
| `$path` | string | Relative path from crate root to the file |
| `$line` | integer | Line where the hook is inserted |
| `$col` | integer | Column where the hook is inserted |
| `$source` | tokens of target expression | The original expression being hooked (for logging) |

The function `load_host_and_port` expands roughly like this (formatting may differ). Using `cargo expand` will show similar output:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/index/tests/snapshots/test__index_expand.snap:12:39}}
```

<div class="warning">

<b>Warning: `line!()` macro is discouraged here.</b>

You might think: "Why not just use `line!()`? Adding a `$line` meta variable increases cognitive load."

This is intentional: `line!()` does not point to the line of the hooked expression; it points to the attribute line (`#[hooq]` or `#[hooq::method(...)]`).

We want the exact line of the `?` or other target, hence `$line` exists.
</div>

Thanks to this expansion, running the program as shown in [Introduction](../index.md) yields output identifying the precise lines where errors occurred:

```bash
$ APP_PORT=10 cargo run -q
{{#include ../../../../../mdbook-source-code/index/tests/snapshots/test__index_with_app_port.snap:8:14}}
```

The default method is helpful, but you will likely want to customize it. We cover customization in the [next lesson](./lesson-02-method.md).
