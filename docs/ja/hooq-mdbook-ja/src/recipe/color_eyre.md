# パーフェクトcolor-eyre

[color-eyre](https://docs.rs/color-eyre/latest/color_eyre/) クレートは、現在Rustに存在する "スタックトレースに類するもの" (BACKTRACEとSPANTRACE) を得る手段としてはおそらく最も進んでおり無視できない存在でしょう。

というわけで、 [公式サンプル](https://docs.rs/color-eyre/latest/color_eyre/#:~:text=%F0%9F%A6%80%20v1.44.0%0A%E2%9D%AF-,RUST_LIB_BACKTRACE%3D1%20cargo%20run%20%2D%2Dexample%20usage,-Finished%20dev%20%5Bunoptimized) に手を加えて、もう一つの "スタックトレースに類するもの" を追加してあげましょう！

新規プロジェクトを `cargo new` で作成し、Cargo.toml は次のようにします。 ( `hooq` の依存については最新バージョンに置き換えてください。 )

```toml
{{#include ../../../../../mdbook-source-code/recipe-color-eyre/Cargo.toml:13:22}}
```

次に、 `::eyre::WrapErr` ではなく `color_eyre::eyre::WrapErr` をuseしたいので、その上書きを行うhooq.tomlをCargo.tomlと同じ階層に置きます。そのほかの設定は組み込みの [eyreフレーバー](../reference/flavors.md#eyre) から引き継がれます！

```toml
{{#include ../../../../../mdbook-source-code/recipe-color-eyre/hooq.toml}}
```

そしてmain.rsを書きます。 `use hooq::hooq;` し、 `#[instrument]` の上に `#[hooq(eyre)]` を付けます[^time]。

[^time]: スナップショットテストの関係で時刻を出力しない `.without_time()` も付与しています。

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/recipe-color-eyre/src/main.rs}}
```

BACKTRACEも見たいので、これを環境変数 `RUST_LIB_BACKTRACE=full` を設定した状態で実行してみます。

[公式例](https://docs.rs/color-eyre/latest/color_eyre/#:~:text=%F0%9F%A6%80%20v1.44.0%0A%E2%9D%AF-,RUST_LIB_BACKTRACE%3D1%20cargo%20run%20%2D%2Dexample%20usage,-Finished%20dev%20%5Bunoptimized)では `.wrap_err()` による付与部分で理由しか書かれていなかった部分も、hooqのおかげでスタックトレースもどきに化けています！

```rust
{{#include ../../../../../mdbook-source-code/recipe-color-eyre/tests/snapshots/test__recipe-color-eyre.snap:9:}}
```

多分これが一番詳細だと思います。
