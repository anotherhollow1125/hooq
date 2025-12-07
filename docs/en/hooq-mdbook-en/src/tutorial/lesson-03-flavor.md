# Create / Use Presets via Flavors

At the end of the [previous lesson](./lesson-02-method.md) we saw `ok_or_else` missing on `Result` when trying to hook uniformly. Can we hook something available on both `Result` and `Option`? The [`Context::with_context`](https://docs.rs/anyhow/latest/anyhow/trait.Context.html#tymethod.with_context) method from [anyhow](https://docs.rs/anyhow/latest/anyhow/) fits perfectly: on `Option` it converts `None` into an `anyhow::Result::Err`.

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/tutorial-4-with-anyhow/src/main.rs}}
```

Run to produce an error: the extra `.with_context(...)` calls accumulate, giving a trace.

```bash
{{#include ../../../../../mdbook-source-code/tutorial-4-with-anyhow/tests/snapshots/test__tutorial-4-with-anyhow.snap:8:18}}
```

Because this is a frequent pattern hooq provides a preset—the [**anyhow flavor**](../reference/flavors.md#anyhow). Presets are called "flavors" in hooq. There are also flavors for [log](../reference/flavors.md#log), [eyre](../reference/flavors.md#eyre--color_eyre), and [tracing](../reference/flavors.md#tracing).

Change `#[hooq]` to `#[hooq(anyhow)]` to enable the flavor:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/tutorial-4-with-anyhow-2/src/main.rs}}
```

Output matches the manual version.

See [Flavors](../reference/flavors.md) for all built‑in presets.

## Define Custom Flavors

You can define flavors in a `hooq.toml` placed at the crate root (`CARGO_MANIFEST_DIR`). Example:

```toml
{{#include ../../../../../mdbook-source-code/tutorial-flavor/hooq.toml}}
```

Meaning of the keys:

| Key | Effect |
|:----|:-------|
| `method` | Method to hook |
| `hook_targets` | Which of `?`, `return`, `tail_expr` to hook |
| `tail_expr_idents` | Idents (e.g. `Err`) that force hooking on tail / return |
| `result_types` | Return type idents (e.g. `Result`) whose tail/return values are considered for hooks |

`my_flavor.ok_or_else` is a **sub‑flavor**, inheriting settings and overriding a subset.

For all available fields and precedence rules see [Attributes](../reference/attributes.md) and [Flavors](../reference/flavors.md).

Use the flavor in code:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/tutorial-flavor/src/main.rs}}
```

`#[hooq::method = flavor_name]` performs partial application of a flavor’s settings. Other fields also support this via `#[hooq::field = flavor_name]`. See [Attributes (partial application)](../reference/attributes.md#partial-application-via-flavor).


<div class="warning">

Subtle note: `$so_far` represents the chain after initial flavor application; you cannot reference `$so_far` inside the base flavor method itself. Thus `#[hooq(my_flavor::ok_or_else)]` cannot include `.$so_far`, while `#[hooq::method = my_flavor::ok_or_else]` can.
</div>
