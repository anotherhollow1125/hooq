# チュートリアル

本章では3ページほどにわたりhooqの基本的な使い方を紹介していきます。

1. [`#[hooq]` を付与しエラー発生行を取得](./lesson-01-hooq.md)
2. [`#[hooq::method(...)]` でフックをカスタム](./lesson-02-method.md)
3. [`フレーバーでプリセットを作成/利用する`](./lesson-03-flavor.md)

hooq クレートは下記の方法で導入済みのものとします。

## hooqマクロの導入方法

hooqクレートは他のクレート同様に `cargo add` で追加できます。

```bash
cargo add hooq
```

Cargo.tomlに最新バージョンを記載することでも可能です。最新バージョンは [crates.io](https://crates.io/crates/hooq) から確認できます。

hooqにもいくつかのfeatureはありますが、よく使うものに関してはdefault featureに含めているので、通常の利用範囲では追加する必要はありません。featureについては [features](../reference/features.md) にまとめています。
