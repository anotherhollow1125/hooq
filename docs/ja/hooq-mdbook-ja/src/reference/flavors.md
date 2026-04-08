# フレーバー

**フレーバー** はhooqマクロのフックするメソッドや挙動等をまとめたプリセットのことを指します。hooqではあらかじめ以下のフレーバーを用意しています。

**組み込みフレーバーのクイックリファレンス**

| フレーバー名 | feature | 内容 |
|:----------|:--------|:----|
| [default](#default) | - | 何も指定しない場合に設定されるフレーバー。hooq.tomlで上書き可 |
| [empty](#empty) | - | 全く何もフックしない場合に用いるフレーバー。上書きは不可 |
| [hook](#hook) | - | [`hooq::HooqMeta`](https://docs.rs/hooq/latest/hooq/struct.HooqMeta.html) を引数に取る `hook` メソッドを挿入するフレーバー。ユーザー定義のトレイト経由での利用を想定。上書き可 |
| [anyhow](#anyhow) | anyhow | [`with_context`](https://docs.rs/anyhow/latest/anyhow/trait.Context.html#tymethod.with_context) メソッドを挿入するフレーバー。上書き可 |
| [eyre](#eyre--color_eyre) / [color_eyre](#eyre--color_eyre) | eyre | [`wrap_err_with`](https://docs.rs/eyre/latest/eyre/trait.WrapErr.html#tymethod.wrap_err_with) メソッドを挿入するフレーバー。上書き可 |
| [log](#log) | log | [`::log::log!`](https://docs.rs/log/latest/log/macro.log.html) を呼び出す `inspect_err` メソッドを挿入するフレーバー。上書き可 |
| [tracing](#tracing) | tracing | [`::tracing::event!`](https://docs.rs/tracing/latest/tracing/macro.event.html) を呼び出す `inspect_err` メソッドを挿入するフレーバー。上書き可 |

一応feature名を記載しましたが、フレーバーに関係するfeatureはdefault featureに含まれているので明示的にCargo.tomlの `features` に含める必要はありません。

フレーバーはユーザーが自分で定義することも可能です。クレートのルート( `CARGO_MANIFEST_DIR` が指し示すディレクトリ) に置いた `hooq.toml` という名前のtomlファイルにて定義します。詳細は次節にて解説します。

## ユーザー定義のフレーバー

hooq.tomlファイルはテーブル名をフレーバー名とし、テーブルに次のフィールドを記述することで設定できます。

| フィールド名 | 型 | 取りうる値の説明 |
|:----------|:---|:---------|
| trait_uses | 文字列配列 | トレイトパス |
| method | 文字列 | フックするメソッド |
| hook_targets | 文字列配列 | `"?"` or `"return"` or `"tail_expr"` |
| tail_expr_idents | 文字列配列 | `"Err"` などフックしたい識別子 |
| ignore_tail_expr_idents | 文字列配列 | `Ok` などフックしたくない識別子 |
| result_types | 文字列配列 | `"Result"` などの返り値型 |
| hook_in_macros | 真偽値 | `true` or `false` |
| bindings | インラインテーブル | 任意のバインディング。**文字列リテラル指定時は `\"` で囲む必要がある点に注意** |

`empty` を除いたhooqで用意している組み込みフレーバーをテーブル名にすることも可能であり、その場合設定値の上書きとなります。

フレーバーテーブルが持つ `bindings` 以外のテーブルは **サブフレーバー** となります。サブフレーバーは親フレーバーの設定値を引き継ぎ、部分的に設定を変更したフレーバーとなります。

設定項目の説明やフレーバーの適用方法等は[属性](./attributes.md)を参照してください。

- [属性](./attributes.md)

hooq.tomlの例:

```toml
{{#include ../../../../../mdbook-source-code/my-flavor/hooq.toml}}
```

使用例:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/my-flavor/src/main.rs}}
```

展開結果:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/my-flavor/src/main.expanded.rs:13:25}}
```

## default

何も指定しなかった場合( `#[hooq]` として付与した場合)の設定となるフレーバーです。

次のような設定になっています。(ドキュメントの整合性を保つためソースコードから直接抜粋しています。以降同様)

```rust
{{#rustdoc_include ../../../../../hooq-macros/src/impls/flavor/mod.rs:91:108}}
```

`default_method()` で設定しているメソッドは次の通りです。(コメントは気にしないでください。)

```rust
{{#rustdoc_include ../../../../../hooq-macros/src/impls/flavor/mod.rs:110:129}}
```

使用例:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/flavor-default/src/main.rs}}
```

実行結果:

```bash
{{#include ../../../../../mdbook-source-code/flavor-default/tests/snapshots/test__flavor-default.snap:8:11}}
```

この設定はhooq.tomlで上書きが可能です。その場合、 `#[hooq]` とした際の設定値が上書きした設定値になります。

## empty

全くフックを行わないフレーバーです。 `#[cfg_attr(feature = "...", hooq(empty))]` のようにコンパイル条件分岐のような特殊な用法を想定しています。(何もフックをしないものの、不活性属性のハンドリングは行うので有用な指定です。)

次のような設定になっています。

```rust
{{#rustdoc_include ../../../../../hooq-macros/src/impls/flavor/presets/empty.rs:10:36}}
```

特殊なフレーバーであるため **唯一上書き不可** としています。

## hook

hookフレーバーの設定は次の通りです。(コメント部分は気にしないでください。)

```rust
{{#rustdoc_include ../../../../../hooq-macros/src/impls/flavor/presets/hook.rs:9:29}}
```


ユーザー側でトレイトを定義し、そこでロギングをする場合に便利なフレーバーです。hooq.tomlを使いたくない際に便利です。

使用例:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/flavor-hook/src/main.rs}}
```

第2引数部分を `meta_fn` というクロージャにしているのは、遅延評価のためです。ここをクロージャにしない場合すべての場所で `HooqMeta` が生成されてしまうため処理が重くなってしまいます。

展開結果:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/flavor-hook/src/main.expanded.rs:37:53}}
```

## anyhow

> `anyhow` feature が必要ですが、defaultに含まれています。

[anyhowクレート](https://docs.rs/anyhow/latest/anyhow/) と共に使うことを想定したフレーバーです。

次の設定になっています。

```rust
{{#rustdoc_include ../../../../../hooq-macros/src/impls/flavor/presets/anyhow.rs:9:37}}
```

[`.with_context(...)`](https://docs.rs/anyhow/latest/anyhow/trait.Context.html#tymethod.with_context) メソッドを利用するために、 `anyhow::Context` トレイトをuseしています。


使用例:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/flavor-anyhow/src/main.rs}}
```

実行結果:

```bash
{{#include ../../../../../mdbook-source-code/flavor-anyhow/tests/snapshots/test__flavor-anyhow.snap:8:19}}
```

## eyre / color_eyre

> `eyre` feature が必要ですが、defaultに含まれています。

[eyreクレート](https://docs.rs/eyre/latest/eyre/) と共に使うことを想定したフレーバーです。anyhowとはuseしているトレイト、呼び出しているメソッドが異なるだけでほぼ同じです。

次の設定になっています。

```rust
{{#rustdoc_include ../../../../../hooq-macros/src/impls/flavor/presets/eyre.rs:9:54}}
```

[`.wrap_err_with(...)`](https://docs.rs/eyre/latest/eyre/trait.WrapErr.html#tymethod.wrap_err_with) メソッドを利用するために、 `eyre::WrapErr` トレイトおよび `eyre::ContextCompat` トレイトをuseしています。

使用例:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/flavor-eyre/src/main.rs}}
```

実行結果:

```bash
{{#include ../../../../../mdbook-source-code/flavor-eyre/tests/snapshots/test__flavor-eyre.snap:8:22}}
```

`use ::eyre::WrapErr as _;` の代わりに `use ::color_eyre::eyre::WrapErr as _;` を挿入する `color_eyre` フレーバーも同時に提供しています。

## log

> `log` feature が必要ですが、defaultに含まれています。

[logクレート](https://docs.rs/log/latest/log/) と共に使うことを想定したフレーバーです。

次の設定になっています。

```rust
{{#rustdoc_include ../../../../../hooq-macros/src/impls/flavor/presets/log.rs:11:67}}
```

デフォルトでは [`log::Level::Error`](https://docs.rs/log/latest/log/enum.Level.html#variant.Error) レベルでログが出力されます。

`#[hooq(log::warn)]` のようにサブフレーバーを利用するか、 `$level` メタ変数に [`log::Level`](https://docs.rs/log/latest/log/enum.Level.html) 列挙型のバリアントを代入することでログレベルを変更できます。

使用例:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/flavor-log/src/main.rs}}
```

実行結果:

```bash
{{#include ../../../../../mdbook-source-code/flavor-log/tests/snapshots/test__flavor-log.snap:8:27}}
```

## tracing

> `tracing` feature が必要ですが、defaultに含まれています。

[tracingクレート](https://docs.rs/tracing/latest/tracing/) と共に使うことを想定したフレーバーです。

次の設定になっています。

```rust
{{#rustdoc_include ../../../../../hooq-macros/src/impls/flavor/presets/tracing.rs:11:74}}
```

デフォルトでは [`tracing::Level::ERROR`](https://docs.rs/tracing/latest/tracing/struct.Level.html#associatedconstant.ERROR) レベルでログが出力されます。

`#[hooq(tracing::warn)]` のようにサブフレーバーを利用するか、 `$level` メタ変数に [`tracing::Level`](https://docs.rs/tracing/latest/tracing/struct.Level.html) 型の値を代入することでログレベルを変更できます。

`#[tracing::instrument]` と併用する場合、 `#[hooq(tracing)]` が先に適用される必要があるため、 `#[tracing::instrument]` より上に `#[hooq(tracing)]` を書くことを推奨します。

使用例:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/flavor-tracing/src/main.rs}}
```

実行結果:

```bash
{{#include ../../../../../mdbook-source-code/flavor-tracing/tests/snapshots/test__flavor-tracing.snap:5:12}}
```
