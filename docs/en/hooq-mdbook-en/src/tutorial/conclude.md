# Summary

Across three lessons we introduced core hooq usage.

There are further mechanisms and details not covered in the tutorial; consult the individual reference pages for deeper exploration.

- [Reference](../reference/index.md): Detailed explanations of each feature.
- [Recipes & Ideas](../recipe/index.md): Usage examples of built‑in flavors, exploratory scenarios, and hidden tricks.

## Comparison With Other Approaches

Applying `#[hooq]` (or `#[hooq(anyhow)]`) to every function yields a pseudo stack trace for errors. Here is a comparison with other strategies for obtaining stack‑like traces:

| | [`Backtrace`](https://doc.rust-lang.org/std/backtrace/struct.Backtrace.html) | [`tracing`](https://docs.rs/tracing/latest/tracing) | `hooq` |
|:-|:-:|:-:|:-:|
| Learning cost / flexibility | ⚠️ | ⚠️ | 🌈 |
| Ease of type definitions | ⚠️ | ✅ | ✅ |
| Macro‑less | 🌈 | ❌ | ❌ |
| Information volume control | ⚠️ | ✅ | 🌈 |
| Platform support | ⚠️ | ✅ | 🌈 |

Legend:
- 🌈: Excellent
- ✅: Good
- ⚠️: So‑so
- ❌: Poor

Explanation:
- Learning cost / flexibility
  - ⚠️ `Backtrace` requires setting `RUST_LIB_BACKTRACE=1` and relies on OS thread info; extra system knowledge helps.
  - ⚠️ `tracing` is powerful but may be overkill if you only want a stack trace analogue.
  - 🌈 `hooq` requires only adding an attribute on functions.
- Ease of type definitions
  - ⚠️ With `thiserror` + `Backtrace`, you must pre‑plan fields; retrofitting is harder as error types multiply.
  - ✅ `tracing` imposes no such constraints.
  - ✅ `hooq` cooperates with arbitrary error crates.
- Macro‑less
  - 🌈 `Backtrace` needs no macros.
  - ❌ `tracing` generally needs `#[tracing::instrument]` for effortless spans.
  - ❌ `hooq` is an attribute macro by design.
- Information volume control
  - ⚠️ Raw `Backtrace` output is often too verbose (and weak for async); pairing with [`color-eyre`](https://docs.rs/color-eyre/latest/color_eyre/) helps.
  - ✅ `tracing` gives structured spans; precise line of each `?` still requires manual logging.
  - 🌈 `hooq` pinpoints only annotated functions and the exact `?` / `return` / tail expression locations.
    - Conditional usage via `#[cfg_attr(..., hooq(...))]` enables feature/test scoped tracing.
    - 💡 Combine with `tracing` to augment granularity—see [tracing flavor](../reference/flavors.md#tracing).
- Platform support
  - ⚠️ `Backtrace` has platform caveats (see [official docs](https://doc.rust-lang.org/std/backtrace/index.html#platform-support) ).
  - ✅ Standard logging with `tracing` is widely portable.
  - 🌈 hooq merely inserts methods; no platform primitives required.
    - 💡 `#[hooq::method(.unwrap()!)]` can alias `?` to `.unwrap()` behind a feature.
