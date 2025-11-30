# Contributing

If you find any improvements, please feel free to open an issue or submit a PR! This page explains the development flow and stance for contributing to hooq.

## About Snapshot Tests

Because hooq is a macro crate, we maintain a large number of snapshots. There are three main ways we take snapshots:

- [macrotest](https://docs.rs/macrotest/latest/macrotest/)
	- Used to obtain the state after macro expansion
	- Most snapshots are taken with this crate
- [insta](https://docs.rs/insta/latest/insta/)
	- Used in mdBook writing to save program stdout/stderr
- Other, custom snapshots
	- Executed in some tests that require a `hooq.toml` file
	- Save `cargo expand` results as-is
	- We couldn’t handle `hooq.toml` well with macrotest

Running tests is simple. From the project root, run `cargo test` to execute all tests in the workspace.

```bash
cargo test
```

If there are differences in snapshots, tests will fail. If the differences are intentional, please take the following actions and re-run the tests:

- For macrotest snapshots and custom snapshots: delete the original snapshot files.
- For insta snapshots: accept the changes via `cargo insta review`.
	- The CLI tool `cargo-insta` is required. Install it with `cargo install cargo-insta`.

## About CI

We use CI to ensure the codebase remains tidy.

- [rust.yml](https://github.com/anotherhollow1125/hooq/blob/main/.github/workflows/rust.yml)

In addition to tests, CI checks the following:

- [cargo +nightly fmt](https://doc.rust-lang.org/cargo/commands/cargo-fmt.html)
	- We use nightly to apply the rules `imports_granularity = "Module"` and `group_imports = "StdExternalCrate"`.
	- See [rust.yml](https://github.com/anotherhollow1125/hooq/blob/main/.github/workflows/rust.yml) for the nightly version.
- [cargo sort --workspace](https://crates.io/crates/cargo-sort)
	- Keeps crate listings in Cargo.toml in a consistent order.
- [cargo clippy](https://github.com/rust-lang/rust-clippy)

## Updating Documentation and the `sync.rs` Command

hooq publishes three crates and provides documentation in both English and Japanese. The latest hooq version is also displayed in the README.

To keep these documents in a correct state, we provide a Cargo Script, [`.github/scripts/sync.rs`](https://github.com/anotherhollow1125/hooq/blob/main/.github/scripts/sync.rs), which is run both locally and in CI.

This script generates each crate’s README.md and the GitHub README.md from the following two template files:

- English: [docs/_readme_root.md.template](https://github.com/anotherhollow1125/hooq/blob/main/docs/_readme_root.md.template)
- Japanese: [docs/ja/_readme_root.md.template](https://github.com/anotherhollow1125/hooq/blob/main/docs/ja/_readme_root.md.template)

Therefore, for example, if you find a typo and want to fix the README.md, please update the template above first, then run the `sync.rs` command to propagate the changes.

Run the following in the repository root to update:

```bash
./.github/scripts/sync.rs
```

## Languages (English/Japanese)

The project owner ([anotherhollow1125](https://github.com/anotherhollow1125), hereafter referred to as “I”) is Japanese, so comments in the codebase are generally written in Japanese. I believe Japanese is well-suited for writing comments, so this policy will not change. Apologies to English speakers.

Issues and PRs in both English and Japanese are welcome! Please use whichever language you prefer.

Most of the English documentation was translated by generative AI. Therefore, if you find awkward terminology usage or nuances in the English docs, please open an issue or submit a PR!

## Stance on Using Generative AI

Recently, “vibe coding” has gained popularity—asking a generative AI to produce entire programs. I am skeptical of this approach and do not use it at all. Please note that PRs created 100% by generative AI are likely to be rejected, except for minor fixes.

That said, I do use generative AI in some ways. The hooq project relies on it quite a bit.

I mainly use generative AI for the following tasks:

- Code completion via [GitHub Copilot](https://github.com/features/copilot)
	- As mentioned, I do not ask an AI to code from scratch in natural language.
	- However, I frequently use Copilot to complete code while I’m actively coding myself.
- Reviews on GitHub via GitHub Copilot
	- I’ve run these on every PR to check for unnatural coding.
- Advice from GitHub Copilot Chat / ChatGPT
	- I consult generative AI to help move the project forward overall.
- Japanese → English translation and documentation generation via GitHub Copilot Chat
- Using [DeepWiki](https://deepwiki.com/) (planned)
	- I plan to create documentation based on existing documents.

On the other hand, I prioritize manual work for the following tasks:

- Development leadership
	- I try to keep Copilot’s completions limited in scope and code while understanding the entire codebase.
- Writing the original Japanese documentation
	- I focus on expressing ideas in my own words, including code examples.
	- As mentioned, once content is more complete, I plan to use tools like DeepWiki for documentation.
