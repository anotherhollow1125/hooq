# `#[hooq]` を付与しエラー発生行を取得

[はじめに](../index.md) で出した次の例を引き続き使って、hooqマクロが一体ソースコードにどのような細工をしたのかを解説し、hooqの使い方を紹介していきます。

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/index/src/main.rs::28}}
```

`#[hooq]` 属性マクロを関数に付与すると、次のhooqマクロデフォルトのメソッド [`inspect_err`](https://doc.rust-lang.org/std/result/enum.Result.html#method.inspect_err) が各 `?` の手前、 `return` の返り値や関数末尾の後ろ(ただし関数シグネチャの返り値型が `Result` などフック対象の型の場合)に挿入(フック)されるようになります。

```rust,ignore
.inspect_err(|e| {
    let path = $path;
    let line = $line;
    let col = $col;
    let expr = ::hooq::summary!($source);

    ::std::eprintln!("[{path}:{line}:{col}] {e:?}\n{expr}");
})
```

`$path` のようなものはhooq固有のメタ変数です。詳細は [メタ変数](../reference/meta_vars.md) にて触れていますが、デフォルトメソッドに登場するものについては以下の通りです。

|メタ変数|リテラル種別|説明|
|:------|:--------|:------|
|`$path`|文字列| クレートルート等からのファイル相対パス |
|`$line`|整数| メソッドがフックされた行 |
|`$col`|整数| メソッドがフックされた列 |
|`$source`| 対象式のトークン列 | フックされる対象の式(ログ表示用) |

`#[hooq]` が施された `load_host_and_port` 関数は次のように展開されます。 `eprintln!` 等まで展開されてしまうため一致はしませんが `cargo expand` で確かめると似たような出力が得られるでしょう。

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/index/tests/snapshots/test__index_expand.snap:12:39}}
```

<div class="warning">
<b><code>line!()</code> マクロは非推奨！</b>

行数取得について、「`line!()` マクロを使えばよいのでは？わざわざ `$line` メタ変数を用意されても認知負荷が上がる」という声をいただきました。

しかしこちらは意図的な設計で、まさしく `line!()` マクロがうまく機能しないことがメタ変数導入のきっかけとなっています。

実際に利用していただくとよくわかりますが、 <b><code>line!()</code>マクロはフックが行われる行を指しません</b>。 `#[hooq]` が存在する行 (あるいは `#[hooq::method(...)]` の行)を出力してしまいます。これは欲しい情報ではないでしょう。ゆえにフックが行われた行を正確に取得するために `$line` メタ変数を設けています。

詳細はこちらの記事に書いています: (WIP)
</div>

`#[hooq]` だけでも上記のように展開されるおかげで、 [はじめに](../index.md) に掲載した次の実行結果が得られ、ソースコードのどの行でエラーが発生したかすぐにわかるようになります。

```bash
$ APP_PORT=10 cargo run -q
{{#include ../../../../../mdbook-source-code/index/tests/snapshots/test__index_with_app_port.snap:8:14}}
```

ところで、デフォルトでフックされるメソッドも十分素敵ですが、カスタムしたいですよね...？メソッドのカスタマイズは [次のレッスン](./lesson-02-method.md) から扱います。