# フレーバー

**フレーバー** はhooqマクロのフックするメソッドや挙動等をまとめたプリセットのことを指します。hooqではあらかじめ以下のフレーバーを用意しています。

**組み込みフレーバーのクイックリファレンス**

| フレーバー名 | feature | 内容 |
|:----------|:--------|:----|
| [default](#default) | - | 何も指定しない場合に設定されるフレーバー。hooq.tomlで上書き可 |
| [empty](#empty) | - | 全く何もフックしない場合に用いるフレーバー。上書きは不可 |
| [hook](#hook) | - | [`hooq::HooqMeta`](https://docs.rs/hooq/0.2.0/hooq/struct.HooqMeta.html) を引数に取る `hook` メソッドを挿入するフレーバー。ユーザー定義のトレイト経由での利用を想定。上書き可 |
| [log](#log) | log | [`::log::error!`](https://docs.rs/log/latest/log/macro.error.html) を呼び出す `inspect_err` メソッドを挿入するフレーバー。上書き可 |
| [anyhow](#anyhow) | anyhow | [`with_context`](https://docs.rs/anyhow/latest/anyhow/trait.Context.html#tymethod.with_context) メソッドを挿入するフレーバー。上書き可 |

一応feature名を記載しましたが、フレーバーに関係するfeatureはdefault featureに含まれているので明示的にCargo.tomlの `features` に含める必要はありません。

フレーバーはユーザーが自分で定義することも可能です。クレートのルート( `CARGO_MANIFEST_DIR` が指し示すディレクトリ) に置いた `hooq.toml` という名前のtomlファイルにて定義します。詳細は [ユーザー定義のフレーバー](#ユーザー定義のフレーバー) の節を参照してください。

## ユーザー定義のフレーバー

## default

## empty

## hook

## log

## anyhow
