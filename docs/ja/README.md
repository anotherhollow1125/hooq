<div align="center">
<a href="https://docs.rs/hooq/0.2.0/hooq/" target="_blank">
<img src="https://raw.githubusercontent.com/anotherhollow1125/hooq/refs/heads/main/assets/hooq_eye_catch3.png" />
</a>
<h1>hooq</h1>

<h3>はてな演算子 (`?`) の前にメソッドを挿入(フック)するシンプルなマクロ</h3>

[![crate](https://img.shields.io/crates/v/hooq)](https://crates.io/crates/hooq)
[![docs](https://img.shields.io/docsrs/hooq/0.2.0)](https://docs.rs/hooq/0.2.0/hooq/)
[![Rust](https://github.com/anotherhollow1125/hooq/actions/workflows/rust.yml/badge.svg)](https://github.com/anotherhollow1125/hooq/actions/workflows/rust.yml)

🪝 hooq という名前は 'HOOk' と 'Question' 演算子 ( `?` ) が由来です 🪝

Enhance your questions by hooq!?

</div>

キーワード: `Result`, `Option`, `hook`, `Result hook`, `Option hook`, `? hook`, `question hook`, `error`, `logging`

ドキュメント:
- チュートリアル: (mdBook を準備中です)
- docs.rs: <https://docs.rs/hooq/0.2.0/hooq/>

<hr />

`#[hooq::method(...)]` で指定したメソッドを式と `?` 演算子 (Question Operator) の間に挿入します！

```rust
use hooq::hooq;

#[hooq]
#[hooq::method(.map(|v| v * 2))]
fn double(s: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let res = s.parse::<u32>()?;
    Ok(res)
}

fn double_expanded(s: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let res = s.parse::<u32>().map(|v| v * 2)?;
    Ok(res)
}

fn main() {
    assert_eq!(double("21").unwrap(), double_expanded("21").unwrap());
}
```

`#[hooq::method(...)]` の指定をわざわざしなくても、予め用意した設定を楽に適用するフレーバーといった仕組みもあります！

## インストール

以下に示すように `cargo add` で加えるか、

```bash
cargo add hooq
```

`Cargo.toml` に加えてください。

```toml
[dependencies]
hooq = "0.2.0"
```

## デフォルトで挿入されるメソッド

`#[hooq]` として特に指定しなければ次のメソッドが挿入されます。

```ignore
.inspect_err(|e| {
    let path = $path;
    let line = $line;
    let col = $col;
    let expr = ::hooq::summary!($source);

    ::std::eprintln!("[{path}:{line}:{col}] {e:?}\n{expr}");
})
```

`#[hooq::method(...)]` 不活性属性でフックするメソッドを切り替えられる他、マクロ呼び出し部を `#[hooq(log)]` や `#[hooq(anyhow)]` としてフレーバーを指定した場合などは、そのフレーバーにちなんだメソッドになります！

## 属性 クイックリファレンス

| 名前 | 種別 | 説明 |
|:----|:----|:----|
| flavor | マクロルートのメタ | 指定したフレーバーの設定を適用する |
| trait_use | マクロルートのメタ | 指定したパス( `XXX` )について `use XXX as _;` をアイテムの前に挿入する |
| method | 不活性属性 | 挿入/置換するメソッド(置換の場合は式)を設定する |
| skip_all / skip | 不活性属性 | 本属性を付与した式へのフックは行わないようになる |
| hook_targets | 不活性属性 | `?`, `return`, 末尾式(tail_expr)それぞれについてフックを行うかを切り替え(デフォルトは3種すべてにフック) |
| tail_expr_idents | 不活性属性 | 末尾式に来た時にフックを行うidentを指定(デフォルトでは `Err` ) |
| ignore_tail_expr_idents | 不活性属性 | フック対象であった場合でもフックを行わないidentを指定(デフォルトでは `Ok` ) |
| result_types | 不活性属性 | `return` と末尾式にフックを行う関数の返り値型を指定(デフォルトは `Result` ) |
| hook_in_macros | 不活性属性 | マクロ内にもフックを行うかを指定(デフォルトは `true` ) |
| binding | 不活性属性 | 指定したリテラル・式で置換されるメタ変数を作成 |

詳細はmdbookの[属性]()を見てください！

## メタ変数 クイックリファレンス

| 名前 | リテラル種別 | 説明 |
|:----|:-----------|:----|
| `$line` | usize整数 | フック対象がある行番号 |
| `$column` or `$col` | usize整数 | フック対象がある列番号 |
| `$path` | 文字列 | フック対象があるファイルの相対パス |
| `$file` | 文字列 | フック対象があるファイルの名前 |
| `$source` | 式 | デバッグ・ロギング用に用いる、挿入/置換対象の式 ( `$expr` との違いに注意 ) |
| `$count` or `$nth` | 文字列 | 何番目の置換対象であるかを表示 |
| `$fn_name` or `$fnname` | 文字列 | フック対象がある関数の名前 |
| `$fn_sig` or `$fnsig` | 文字列 | フック対象がある関数のシグネチャ |
| `$xxx` (一例) | (任意) | `#[hooq::xxx = ...]` という不活性属性によるユーザー定義のメタ変数 |
| `$bindings` or `$vars` | [`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html) | メタ変数バインディングすべて |
| `$hooq_meta` or `$hooqmeta` | [`hooq::HooqMeta`](https://docs.rs/hooq/latest/hooq/struct.HooqMeta.html) | `$line`・`$col`・`$path`・`$file`・`$source`・`$count`・`$bindings` をひとまとめにした構造体を表す |
| `$expr` | 式 | 置換用に用いる、置換対象の式 ( `$source` との違いに注意 ) |
| `$so_far` or `$sofar` | 式 | 主に挿入用に用いる、これまでに設定されているフック |

詳細はmdbookの[メタ変数]()を見てください！

## 組み込みフレーバー クイックリファレンス

| フレーバー名 | feature | 内容 |
|:----------|:--------|:----|
| default | - | 何も指定しない場合に設定されるフレーバー。hooq.tomlで上書き可 |
| empty | - | 全く何もフックしない場合に用いるフレーバー。上書きは不可 |
| hook | - | [`hooq::HooqMeta`](https://docs.rs/hooq/latest/hooq/struct.HooqMeta.html) を引数に取る `hook` メソッドを挿入するフレーバー。ユーザー定義のトレイト経由での利用を想定。上書き可 |
| log | log | [`::log::error!`](https://docs.rs/log/latest/log/macro.error.html) を呼び出す `inspect_err` メソッドを挿入するフレーバー。上書き可 |
| anyhow | anyhow | [`with_context`](https://docs.rs/anyhow/latest/anyhow/trait.Context.html#tymethod.with_context) メソッドを挿入するフレーバー。上書き可 |
| eyre | eyre | (WIP) |
| tracing | tracing | (WIP) |

詳細はmdbookの[フレーバー]()を見てください！
