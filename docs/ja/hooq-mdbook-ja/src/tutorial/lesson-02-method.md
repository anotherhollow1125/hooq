# `#[hooq::method(...)]` でフックをカスタム

フックされるメソッドはユーザーが設定することが可能です。

プロジェクトのCargo.tomlの `name` を表示するプログラム例で説明します。tomlファイルを読めるようにしたいためtomlクレートを導入します。

```bash
cargo add toml
```

ひとまず指定されたパスのファイルをtomlファイルとして読み取る機構を作ります。

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/tutorial-1/src/main.rs:7:15}}
```

そのままhooqマクロを適用しても良いですが、今回はフックをカスタムしてみるというお題なのでやってみましょう。エラーなくうまくいった場合もわかるように、 [`inspect`](https://doc.rust-lang.org/std/result/enum.Result.html#method.inspect) メソッドも追加してみます。

フックするメソッドを指定したい時は、 `#[hooq::method(...)]` 属性を使います。現在設定済みのフックメソッドを表す `.$so_far` メタ変数を使うことで、追加分として `inspect` メソッドを入れることができます。

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/tutorial-1/src/main.rs::15}}
```

失敗の場合いままで通りログが出ます。

```bash
{{#include ../../../../../mdbook-source-code/tutorial-1/tests/snapshots/test__tutorial-1-not-exist.snap:8:11}}
```

成功の場合もログを出力するようになりました！

```bash
{{#include ../../../../../mdbook-source-code/tutorial-1/tests/snapshots/test__tutorial-1.snap:6:7}}
```

次は名前を表示する機構を作りましょう。tomlから目的のフィールドを `toml::Value::get` を使うことで抽出していきます。 `toml::Value::get` は `Option` を返します。[`ok_or_else`](https://doc.rust-lang.org/std/option/enum.Option.html#method.ok_or_else) を使うことで、 `Option` 型を `Result` 型に変換することが可能なので、今回はそのようにします。

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/tutorial-2/src/main.rs}}
```

...このような恣意的な例を持ち出して私が何を言いたいか、もうお分かりですね？ `.ok_or_else(...)` はボイラープレートです！hooqマクロを使えばこの記述は簡略化できます！

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/tutorial-3/src/main.rs:3:17}}
```

今回、何番目の `?` 演算子であるかを示す `$nth` メタ変数も入れてみました。エラーになるように実行させると次のようになります。

```bash
{{#include ../../../../../mdbook-source-code/tutorial-3/tests/snapshots/test__tutorial-3-name-missing.snap:5:10}}
```

`package` フィールドはあるものの `name` フィールドがないtomlを読ませました。想定通り、2番目の `?` でエラーになったことが示されています。

## `#[hooq::skip_all]` でフックをスキップ

「4は `404` 等を連想させて不吉[^mista]だからタイトルに4が入っているかチェックするぜ！」と言われたのでバリデーションを付けることになりました。こんな感じでしょうか？

[^mista]: でもこれで行くとサーバーサイドエンジニアなどには `5` の方が不吉そうですね。

```rust,ignore
{{#rustdoc_include ../../../../../mdbook-source-code/tutorial-4-compile-error/src/_main.txt:3:23}}
```

hooqは `return Err(...);` にもメソッドをフックすることを試みるため、 `Result` 型に `ok_or_else` メソッドが存在しないというコンパイルエラーになってしまいます[^error]。

[^error]: hooqマクロのSpan設定の影響でエラーの補足がおかしくなってしまっていますね。もう少しマシなSpanに直したいと思います。

```bash
{{#include ../../../../../mdbook-source-code/tutorial-4-compile-error/tests/snapshots/test__tutorial-4-compile-error.snap:8:21}}
```

ここは今回はフックしたくない場所です。そのような場合は `#[hooq::skip_all]` を付与することで、フックさせないようにできます！

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/tutorial-4/src/main.rs:3:24}}
```

<details><summary>完成したプログラム全体</summary>

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/tutorial-4/src/main.rs}}
```
</details>

無事にコンパイルが通り、名前に4が入っている時はエラーとなります。

```bash
{{#include ../../../../../mdbook-source-code/tutorial-4/tests/snapshots/test__tutorial-4.snap:5:10}}
```

`#[hooq::skip_all]` の他にも、途中でhooqの挙動を変更するための属性はいくつかあります。  `#[hooq::method(...)]` もその一つで、関数の冒頭だけではなく、個別の式や文に付与することも可能です。詳細が知りたい方は[属性](../reference/attributes.md)を参照してください。

ここまでカスタムフックを設定する方法を解説しました。しかし関数ごとに毎回 `#[hooq::method(...)]` で目当てのメソッドを設定するのはちょっと面倒ですよね？

[次のレッスン](./lesson-03-flavor.md)ではあらかじめカスタムフックを設定しておいたり、hooqクレート自体が用意している便利な設定を適用するための **フレーバー機能** について解説したいと思います。
