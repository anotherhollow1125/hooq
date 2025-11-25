# メソッド

不活性属性 `#[hooq::method(...)]` やフレーバーを通して、hooqによってフックされるメソッドを指定することが可能です。 `...` の部分に指定するメソッドには2つのモードがあります。

`#[hooq::method(.inspect_err(|_| ...))]` のように `.` 始まりで記述されている場合、 **挿入モード** となり、付与対象の式と `?` の間に式が挿入されるようになります。 `#[hooq::method(wrapper($expr))]` のように `.` が冒頭にはない場合、 **置換モード** となり、付与対象の式が指定されたメソッド(というよりは変換後の式)に置換されます。

挿入/置換先の式表現には、 `$` マークで始まる[メタ変数](./meta_vars.md)が使えます。詳細はメタ変数のページを確認してください。

- [メタ変数](./meta_vars.md)

## 挿入モード

`.` (ドット)で始まる場合は内容がメソッドであると判断し、設定されたメソッドを挿入します。hooqの基本的なモードです。

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/method_insert_mode/src/main.rs}}
```

`.$so_far` を使うことで、それ以前に設定されたメソッドにつなげたり、逆にそのメソッドを後ろに付けたりすることも可能です[^sofar]。

[^sofar]: `$so_far` は前回のフックが挿入モードの場合先頭のドットを落としてメソッドを保存しているので、 `.` を先頭に付ける仕組みになっています。チェインが視覚的にわかるためこのようにしています。

展開結果は次のようになります。

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/method_insert_mode/src/main.expanded.rs:10:30}}
```

## 置換モード

`.` (ドット)で始まらない場合は内容が置換される関数であると判断し、設定された式で元の式を置換します。 置換される対象の式は `$expr` メタ変数を利用してアクセスします。( `$expr` を使わずに書くことも可能ですが、おそらく役には立たないでしょう。)

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/method_replace_mode/src/main.rs}}
```

展開された `main` 関数内では対象の式が置換されています。

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/method_replace_mode/src/main.expanded.rs:21:24}}
```

<div class="warning">

**置換モードでは `$expr` を利用してください！**

`$expr` メタ変数も `$source` メタ変数もともにフックされる元の式を表します。ただし、 `$expr` メタ変数は置換モード等で元の式を参照するために、 `$source` メタ変数はデバッグやロギングにおいてフックされる式を参照するためにそれぞれ用います。

両者の違いは、式の内側へのフックの有無です。 `$expr` は内側へのフックを済ませてから再帰的に現在のフック処理に渡されているため、フックがない元々の式にはなっていません。一方で `$source` はフックが施される前の元々の式を表すので、置換モードにて(ロギング文字列を得る目的以外で)利用するとフックが適切に行われなくなります。
</div>

## `!` (Exclamation Mark) の末尾付与による `?` (Question Operator) の削除

`?` 演算子 (Question Operator) へフックする場合に限り、フックするメソッドの末尾に `!` (Exclamation Mark) を付与すると末尾の `?` 演算子を取り除く機能があります。

> 本機能を利用するには **[`consume-question`](./features.md) featureが必要** です。

```bash
cargo add hooq --features consume-question
```

例えば以下のように `?` を `.unwrap()` として扱いたい場合などに便利です。

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/consume_question/src/main.rs}}
```

展開結果は次のようになります。

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/consume_question/src/main.expanded.rs:6:12}}
```