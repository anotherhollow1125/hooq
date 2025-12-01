# 特定のfeatureが有効時に .unwrap() にする

[メソッド](../reference/method.md#-exclamation-mark-の末尾付与による--question-operator-の削除) にて説明している通り、 `consume-question` feature を有効にしている時に限り、 `!` をメソッド末尾に付けることで `?` を削除することが可能です。

これを利用すると、例えば `?` を `unwrap` の別名として利用できたりします。

例として、 `unwrap` featureが有効な時だけ `?` を `.unwrap()` に置き換える例を紹介します。Cargo.tomlのfeaturesが以下のように設定されているとします。

```toml
[dependencies]
hooq = { version = "*", features = ["consume-question"] }

# ..

{{#include ../../../../../mdbook-source-code/recipe-unwrap/Cargo.toml:13:14}}
```

hooq.tomlに予めフレーバーを用意しておきます。末尾式や `return` に同様の置換を行われると困るため、 `hook_targets` の方で `?` でのみ置き換わるように指定しておきます。

```toml
{{#include ../../../../../mdbook-source-code/recipe-unwrap/hooq.toml}}
```

main.rsを次のようにします。

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/recipe-unwrap/src/main.rs}}
```

`unwrap` featureがついていない時の展開結果は、[empty](../reference/flavors.md#empty)フレーバーが適用されるので何も変化が起きていません。

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/recipe-unwrap/src/main.expanded.rs:6:20}}
```

一方で、 `unwrap` featureをつけている時の展開結果では、狙い通り `.unwrap()` に置き換わっています！

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/recipe-unwrap/src/main.unwrap.expanded.rs:6:20}}
```

本節では2つのことを紹介しました。

- `?` を `.unwrap()` など特殊なものにも置き換えられること
- `#[cfg_attr(..., hooq(...))]` でfeatureによってフック内容を変えられること

前者は(作者は詳しくありませんが、)unsafe Rustやno_std環境などでも `?` を導入できる足がかりにできるかもしれません。後者は今回の例に限らず、featureを利用してデバッグ情報を調整する手段として有効活用できそうです！
