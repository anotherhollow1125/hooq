# Features

The hooq crate provides several Cargo features:

| Feature | default | Description |
|:--------|:------:|:------------|
| default | ✓ | Default set; includes all except `consume-question`. |
| full | ✗ | Includes all features. |
| anyhow | ✓ | Provides the `anyhow` flavor. |
| eyre | ✓ | Provides the `eyre` flavor. |
| log | ✓ | Provides the `log` flavor. |
| tracing | ✓ | Provides the `tracing` flavor. |
| consume-question | ✗ | Allows removing the `?` operator via a trailing `!` on hooked methods.

Only `consume-question` is excluded from `default`. Flavor‑related features are included by default.
