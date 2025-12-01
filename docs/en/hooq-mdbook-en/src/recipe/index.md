# Recipes & Ideas

This chapter collects smaller ideas enabled by hooq.

Practical usage has already been shown in the [Tutorial](../tutorial/index.md). These entries may be less practical but explore possibilities and might inspire new use cases.

## Contents

| Page | Summary |
|:-----|:--------|
| [Perfect color-eyre](./color_eyre.md) | Use [`color-eyre`](https://docs.rs/color-eyre/latest/color_eyre/) for `BACKTRACE`/`SPANTRACE`, while also getting hooq’s pseudo trace for maximum insight. |
| [Batch apply under a module](./batch_apply.md) | hooq can be attached to items like `mod` to hook inner functions recursively. |
| [Using in functions returning Option](./option.md) | Adjust `tail_expr_idents` and `result_types` to target `Option` returns. |
| [Turn `?` into `.unwrap()` behind a feature](./unwrap.md) | Use `#[cfg_attr(...)]` to change behavior conditionally. |
| [Reinvent `match` desugaring](./match.md) | Replace `?` with `match` using `consume-question` + custom flavor. |
