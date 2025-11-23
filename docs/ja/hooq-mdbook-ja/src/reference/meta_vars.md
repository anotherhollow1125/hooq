# メタ変数

`#[hooq::method(...)]` の `...` 内部では特殊な値に置換される **メタ変数** を利用することが可能です。メタ変数は `$` で始まります。

**クイックリファレンス**

| 名前 | リテラル種別 | 説明 |
|:----|:-----------|:----|
| [`$line`](#line) | usize整数 | フック対象がある行番号 |
| [`$column`](#column) or [`col`](#column) | usize整数 | フック対象がある列番号 |
| [`$path`](#path) | 文字列 | フック対象があるファイルの相対パス |
| [`$file`](#file) | 文字列 | フック対象があるファイルの名前 |
| [`$source`](#source) | 式 | デバッグ・ロギング用に用いる、挿入/置換対象の式 ( [`$expr`](#expr) との違いに注意 ) |
| [`$count`](#count) or [`$nth`](#count) | 文字列 | 何番目の置換対象であるかを表示 |
| [`$fn_name`](#fn_name) or [`$fnname`](#fn_name) | 文字列 | フック対象がある関数の名前 |
| [`$fn_sig`](#fn_sig) or [`$fnsig`](#fn_sig) | 文字列 | フック対象がある関数のシグネチャ |
| [`$expr`](#expr) | 式 | 置換用に用いる、置換対象の式 ( [`$source`](#source) との違いに注意 ) |
| [`$so_far`](#so_far) or [`$sofar`](#so_far) | 式 | 主に挿入用に用いる、これまでに設定されているフック |
| [`$bindings`](#bindings) or [`vars`](#bindings) | (任意) | 任意のメタ変数バインディング |
| [`$hooq_meta`](#hooq_meta) or [`$hooqmeta`](#hooq_meta) | [`hooq::HooqMeta`](https://docs.rs/hooq/0.2.0/hooq/struct.HooqMeta.html) | `$line`・`$col`・`$path`・`$file`・`$source`・`$count`・`$bindings` をひとまとめにした構造体を表す |

## フック対象の情報を得るためのメタ変数

### line

### column

### path

### file

### source

### count

### fn_name

### fn_sig

## 高度なフックを作成するためのメタ変数

### expr

### so_far

### bindings

### hooq_meta


