# Option型を返す関数での利用

[属性でフック対象判定を制御する](../reference/attributes.md#フック対象判定を制御する) 方法を紹介していましたが、イマイチピンとこなかったと思います。そもそも滅多に触ることはない設定だろうなと作者も考えています。

それでもなるべくイメージが付きやすい例ということで、 ( `Result` 型ではなく) `Option` 型にフックすることのみを目的とした設定を施してみましょう。

次の例では、main関数内に定義した関数にフックをしています。(ネストできることについては [mod以下の関数に一括適用](./batch_apply.md)を参照してください。) `Some` であったときに中身を確認するメソッドを挿入したいというシナリオです。

- `tail_expr_idents`: 今回、 `Some` にのみフックしたいので `Some` を指定
- `ignore_expr_idents`: `None` にフックを仕掛けるのは無駄なので(ほかに条件がそろっている時でも) `None` にはフックしないようにする
- `result_types`: `Option` 型関数の時は末尾式や `return` に来る値の型をフック対象だとみなすようになる

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/recipe-option/src/main.rs}}
```

前レシピの[mod以下の関数に一括適用](./batch_apply.md)と同じぐらいのパズルになってしまいましたね...

展開結果は次のようになります。

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/recipe-option/src/main.expanded.rs:8:27}}
```

ともかく本節で主張したかったのは、基本的にはフック対象の型や識別子は `Result`, `Ok`, `Err` から変更することはないかもしれませんが、それでもhooqは対象となる型・識別子の設定を完全に固定しているわけではなく、一応設定変更可能にしているということです。

将来的に [Tryトレイト](https://doc.rust-lang.org/std/ops/trait.Try.html) 周りがstableになった時などに利用の幅が広がればと考えています。
