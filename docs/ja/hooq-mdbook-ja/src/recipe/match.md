# match 脱糖の再発明

[特定のfeatureが有効時に .unwrap() にする](./unwrap.md) にて `consume-question` feature有効時に `!` を使うことで `?` を別なものに置換できることを示しました。

これを利用すると `?` を `match` に置き換えることも可能です。 `match` 脱糖の再発明ですね。面白い例なので載せておこうと思います。

まず、忘れずに `consume-question` を有効にしておいてください。

```bash
cargo add hooq --features consume-question
```

hooq.tomlにて置換結果となる `match` 式を定義します。 `$expr` が置換元の式を表します！フレーバー名ですが、 `match` は予約語なので `my_match` としています。

```toml
{{#include ../../../../../mdbook-source-code/recipe-match/hooq.toml}}
```

後は、いつも通りmain.rsを書くだけです。ただし再発明はClippyに怒られる可能性があるので、一応 `#[allow(clippy::question_mark)]` を関数の頭に付けましょう。

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/recipe-match/src/main.rs}}
```

展開結果においては狙い通り `match` 式になっています！

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/recipe-match/src/main.expanded.rs:6:17}}
```
