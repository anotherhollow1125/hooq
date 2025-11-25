# メタ変数

`#[hooq::method(...)]` の `...` 内部では特殊な値に置換される **メタ変数** を利用することが可能です。メタ変数は `$` で始まります。

**クイックリファレンス**

| 名前 | リテラル種別 | 説明 |
|:----|:-----------|:----|
| [`$line`](#line) | usize整数 | フック対象がある行番号 |
| [`$column`](#column) or [`$col`](#column) | usize整数 | フック対象がある列番号 |
| [`$path`](#path) | 文字列 | フック対象があるファイルの相対パス |
| [`$file`](#file) | 文字列 | フック対象があるファイルの名前 |
| [`$source`](#source) | 式 | デバッグ・ロギング用に用いる、挿入/置換対象の式 ( [`$expr`](#expr) との違いに注意 ) |
| [`$count`](#count) or [`$nth`](#count) | 文字列 | 何番目の置換対象であるかを表示 |
| [`$fn_name`](#fn_name) or [`$fnname`](#fn_name) | 文字列 | フック対象がある関数の名前 |
| [`$fn_sig`](#fn_sig) or [`$fnsig`](#fn_sig) | 文字列 | フック対象がある関数のシグネチャ |
| `$xxx` (一例) | (任意) | `#[hooq::xxx = ...]` という不活性属性によるユーザー定義のメタ変数 |
| [`$bindings`](#bindings) or [`$vars`](#bindings) | [`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html) | メタ変数バインディングすべて |
| [`$hooq_meta`](#hooq_meta) or [`$hooqmeta`](#hooq_meta) | [`hooq::HooqMeta`](https://docs.rs/hooq/latest/hooq/struct.HooqMeta.html) | `$line`・`$col`・`$path`・`$file`・`$source`・`$count`・`$bindings` をひとまとめにした構造体を表す |
| [`$expr`](#expr) | 式 | 置換用に用いる、置換対象の式 ( [`$source`](#source) との違いに注意 ) |
| [`$so_far`](#so_far) or [`$sofar`](#so_far) | 式 | 主に挿入用に用いる、これまでに設定されているフック |

`$expr`、 `$so_far` は特殊なフックを作るため一旦除くとして、それ以外のすべてのメタ変数を利用した例は以下になります。

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/meta-vars-all/src/main.rs}}
```

展開されるとこのようになります。

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/meta-vars-all/src/main.expanded.rs:10:66}}
```

## フック対象の情報を得るためのメタ変数

フック対象が存在する行数・列数・関数名・ファイル名・ファイルパス・フック対象のトークン列・何番目の `?` であるか等、フック対象の情報を表すメタ変数を紹介します。

### line

| 名前 | リテラル種別 | 説明 |
|:----|:-----------|:----|
| `$line` | usize整数 | フック対象がある行番号 |

フック対象が何行目にあるかを表すusize整数に置き換えられます。

<div class="warning">

[`line!()` マクロ](https://doc.rust-lang.org/std/macro.line.html)は正確な行数を表示しないため、行数を得たい場合は `$line` を利用してください。
</div>

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/meta-vars-line/src/main.rs}}
```

展開結果:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/meta-vars-line/src/main.expanded.rs:10:16}}
```

### column

| 名前 | リテラル種別 | 説明 |
|:----|:-----------|:----|
| `$column` or `$col`  | usize整数 | フック対象がある列番号 |

フック対象が何列目にあるかを表すusize整数に置き換えられます。

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/meta-vars-column/src/main.rs}}
```

展開結果:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/meta-vars-column/src/main.expanded.rs:10:16}}
```

### path

| 名前 | リテラル種別 | 説明 |
|:----|:-----------|:----|
| `$path` | 文字列 | フック対象があるファイルの相対パス |

フック対象が存在するファイルの、クレートルート( `CARGO_MANIFEST_DIR` )からの相対パスに置き換えられます。

しかしながら相対パスの起点は、ワークスペースを利用している場合などにクレートルートではない時があります。手続きマクロからはファイルの正確な絶対パスが取れない仕組みになっているらしいため、hooqでは絶対パスを取得するためのメタ変数は設けられていません。

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/meta-vars-path/src/main.rs}}
```

展開結果:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/meta-vars-path/src/main.expanded.rs:10:16}}
```

### file

| 名前 | リテラル種別 | 説明 |
|:----|:-----------|:----|
| `$file` | 文字列 | フック対象があるファイルの名前 |

フック対象が存在するファイルの名前に置き換えられます。

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/meta-vars-file/src/main.rs}}
```

展開結果:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/meta-vars-file/src/main.expanded.rs:10:16}}
```

### source

| 名前 | リテラル種別 | 説明 |
|:----|:-----------|:----|
| `$source` | 式 | デバッグ・ロギング用に用いる、挿入/置換対象の式 |

hooqマクロによるフックが一切施される前のフック対象トークン列(式)を得られます。デバッグ用途での使用を想定したメタ変数です。

<div class="warning">

一方で `$expr` は内部にすでにフックが施された状態の式です。 `$expr` は置換モード等で元の式を埋め込む場所を決めるために使用されます。
</div>

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/meta-vars-source/src/main.rs}}
```

展開結果:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/meta-vars-source/src/main.expanded.rs:10:16}}
```

`$source` メタ変数は単体ではなく以下の文字列化マクロと併用することを想定して作られています。

- [`stringify!` マクロ](https://doc.rust-lang.org/std/macro.stringify.html)
- [`hooq::summary!` マクロ](https://docs.rs/hooq/latest/hooq/macro.summary.html)
- [`hooq::source_excerpt_helpers`](https://docs.rs/hooq/latest/hooq/source_excerpt_helpers/index.html) モジュール以下にあるマクロ

`hooq::summary!` マクロを使うと程よく改行が施され、読みやすいエラーメッセージを作成できます。

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/meta-vars-source-summary/src/main.rs}}
```

実行結果(`eprintln!` による表示部分):

```bash
{{#include ../../../../../mdbook-source-code/meta-vars-source-summary/tests/snapshots/test__meta-vars-source-summary.snap:8:14}}
```

### count

| 名前 | リテラル種別 | 説明 |
|:----|:-----------|:----|
| `$count` or `$nth` | 文字列 | 何番目の置換対象であるかを表示 |

フック対象がそれぞれの種別( `?` か、 `return` か、末尾式か)で関数内において何番目であるかを指し示します。

本メタ変数は `$line` の取得がnightlyでしか不可能であった時の開発の名残です。 `$line` の方が基本的にはわかりやすいでしょう。

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/meta-vars-count/src/main.rs}}
```

展開結果:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/meta-vars-count/src/main.expanded.rs:10:16}}
```

### fn_name

| 名前 | リテラル種別 | 説明 |
|:----|:-----------|:----|
| `$fn_name` or `$fnname` | 文字列 | フック対象がある関数の名前 |

フック対象が存在する関数名に置き換わります。ネストされている場合一番内側の関数名を指し、クロージャ内の場合は関数の中のクロージャであることが明記されます。

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/meta-vars-fn_name/src/main.rs}}
```

展開結果:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/meta-vars-fn_name/src/main.expanded.rs:10:26}}
```

### fn_sig

| 名前 | リテラル種別 | 説明 |
|:----|:-----------|:----|
| `$fn_sig` or `$fnsig` | 文字列 | フック対象がある関数のシグネチャ |

フック対象が存在する関数・クロージャのシグネチャに置き換わります。

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/meta-vars-fn_sig/src/main.rs}}
```

展開結果:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/meta-vars-fn_sig/src/main.expanded.rs:10:26}}
```

## ユーザー定義のメタ変数(バインディング)

メタ変数は、組み込み以外の名前ならばユーザーにより定義することが可能です。

`#[hooq::xxx = ...]` のように不活性属性を用いるか、hooq.tomlファイル内の `bindings` テーブルに書くことでメタ変数を定義できます。

定義方法の詳細はそれぞれのページを参照してください。

- [binding (不活性属性)](./attributes.md#binding)
- [フレーバー](./flavors.md#ユーザー定義のフレーバー)

本ページでは不活性属性を用いた例を掲載します。

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/meta-vars-binding/src/main.rs}}
```

式であればなんでも設定可能です。ただしその式にそのまま置き換わるだけなので、書き方によっては所有権等の影響を受けることもあるでしょう。

展開結果:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/meta-vars-binding/src/main.expanded.rs:14:28}}
```

### bindings

| 名前 | リテラル種別 | 説明 |
|:----|:-----------|:----|
| `$bindings` or `$vars` | [`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html) | メタ変数バインディングすべて |

ユーザー定義のメタ変数(バインディング)を [`HashMap<String, BindingPayload>`](https://doc.rust-lang.org/std/collections/struct.HashMap.html) としてすべて取得します。

[`BindingPayload`](https://docs.rs/hooq/latest/hooq/struct.BindingPayload.html) にはバインディングの式を文字列化したものと、 `Rc<dyn Any>` を用いた値が保存されています。

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/meta-vars-bindings/src/main.rs}}
```

展開結果:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/meta-vars-bindings/src/main.expanded.rs:14:63}}
```

### hooq_meta

| 名前 | リテラル種別 | 説明 |
|:----|:-----------|:----|
| `$hooq_meta` or `$hooqmeta` | [`hooq::HooqMeta`](https://docs.rs/hooq/latest/hooq/struct.HooqMeta.html) | `$line`・`$col`・`$path`・`$file`・`$source`・`$count`・`$bindings` をひとまとめにした構造体を表す |

[`hook` フレーバー](./flavors.md#hook) のために設けられたメタ変数で、メタ変数として得られる情報を[`hooq::HooqMeta`](https://docs.rs/hooq/latest/hooq/struct.HooqMeta.html)としてまとめます。

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/meta-vars-hooq_meta/src/main.rs}}
```

展開結果:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/meta-vars-hooq_meta/src/main.expanded.rs:10:38}}
```

[`hook` フレーバー](./flavors.md#hook)の方も合わせてご確認ください。

- [`hook` フレーバー](./flavors.md#hook)

## 高度なフックを作成するためのメタ変数

ここまで紹介したメタ変数は基本的にはロギングやデバッグ等でメタ情報を得るためのものでした。

残りの2つ `$expr` および `$so_far` はメソッドの記述を補助するメタ変数になります。

### expr

| 名前 | リテラル種別 | 説明 |
|:----|:-----------|:----|
| `$expr` | 式 | 置換用に用いる、置換対象の式 |

内部に対しhooqマクロによるフックが施された後のフック対象トークン列(式)を得られます。[メソッドの置換モード](./method.md#置換モード) において、フック対象(置換対象)を表すのに用いるメタ変数です。

<div class="warning">

先に説明した通り `$source` の方は一切フックが施されていない状態の式です。 `$source` はデバッグ・ロギング出力で元の式を表示するのに使用します。
</div>

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/method_replace_mode/src/main.rs}}
```

展開結果:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/method_replace_mode/src/main.expanded.rs:21:24}}
```

### so_far

| 名前 | リテラル種別 | 説明 |
|:----|:-----------|:----|
| `$so_far` or `$sofar` | 式 | 主に挿入用に用いる、これまでに設定されているフック |

すでに設定済みのフックを表すメタ変数です。途中で挿入するメソッドを追加したくなった際などに用いることができます。

挿入モード用のフックが入っている場合、意図的に `$so_far` の中身からは先頭の `.` (ドット)を抜いてあるので、挿入したい箇所では `.$so_far` のようにドットを付けて記述します。

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/method_insert_mode/src/main.rs}}
```

展開結果:

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/method_insert_mode/src/main.expanded.rs:10:30}}
```
