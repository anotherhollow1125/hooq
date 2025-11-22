# はじめに

[hooq](https://crates.io/crates/hooq) は、 `?` 演算子[^try]、 `return` 、末尾の式などに対し、必要に応じてメソッドを挿入できる属性マクロです。

[^try]: 旧tryマクロ

```rust
{{#rustdoc_include ../../../../mdbook-source-code/eye_catch/src/main.rs::18}}
```

豊富なプリセットがあり、エラーロギング等をお手軽に `Result` にフックすることができます。

## なぜhooqを使うか?

hooqのモチベーションを、次のような `Result` 型を返す関数を含むソースコードを例に説明します。

```rust
{{#rustdoc_include ../../../../mdbook-source-code/index_without_hooq/src/main.rs::24}}
```

各環境変数を指定して実行するとエラーなく実行されます。

```bash
$ APP_HOST=127.0.0.1 APP_PORT=10 cargo run -q
{{#include ../../../../mdbook-source-code/index_without_hooq/tests/snapshots/test__index_without_hooq_with_envs.snap:6:6}}
```

このプログラムがエラーとなるように実行してみます。

```bash
$ APP_PORT=10 cargo run -q
{{#include ../../../../mdbook-source-code/index_without_hooq/tests/snapshots/test__index_without_hooq_with_app_port.snap:8:8}}
```

`main` 関数が返した `Box<dyn Error>` の内容が表示され、(おそらく)何かしらの環境変数が足りていないというエラーが発生しています。

しかしこのエラー表示は酷いです！

- どういうコンテキストのなんのエラーなのかがわからない
- **エラーが発生した場所がわからない**

おそらくこのRustプログラムを書いた人は、きめ細かいエラーハンドリングを含んだフォーマルなアプリケーションを作りたかったのではなく、ちょっとしたカジュアルなCLIツールを作りたかったのだと思います。しかしRustのエラー表示はサボる人には冷たいものです[^unwrap]。

[^unwrap]: この規模なら `unwrap` を使えばよいだろうって...？君のような勘のいいガキは嫌いだよ...。そうは言っても `Result` 型が使えないのは不便なはずなので、hooqマクロの意義は依然としてあるでしょう。

**こんな時に使えるのが `hooq` 属性マクロです！**

```rust
{{#rustdoc_include ../../../../mdbook-source-code/index/src/main.rs::28}}
```

`#[hooq]` を付けるだけであら不思議、エラーのスタックトレースもどきが出力されるようになります！

```bash
$ APP_PORT=10 cargo run -q
{{#include ../../../../mdbook-source-code/index/tests/snapshots/test__index_with_app_port.snap:8:14}}
```

どうやら `APP_HOST` 環境変数が足りていなかったようですね。エラー発生箇所についても、8行目、21行目と伝搬していったということがわかります。

一体 `hooq` マクロが何をしたのか、それは [後ほど](./tutorial/index.md) 解説していきます。