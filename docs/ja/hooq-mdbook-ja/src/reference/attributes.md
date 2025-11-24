# 属性

hooqマクロの挙動は、 `#[hooq(ルートメタ)]` におけるルートメタおよび、途中に挿入される不活性属性 ( `#[hooq::ATTR(...)]` ) によって変更できます。

本ページでは属性による設定可能項目および設定値のデフォルトを解説します。

**クイックリファレンス**

| 名前 | 種別 | 説明 |
|:----|:----|:----|
| [flavor](#flavor) | マクロルートのメタ | 指定したフレーバーの設定を適用する |
| [trait_use](#trait_use) or [trait_uses](#trait_use) | マクロルートのメタ | 指定したパス( `XXX` )について `use XXX as _;` をアイテムの前に挿入する |
| [method](#method) | 不活性属性 | 挿入/置換するメソッド(置換の場合は式)を設定する |
| [skip_all](#skip_all) / [skip](#skip) | 不活性属性 | 本属性を付与した式へのフックは行わないようになる |
| [hook_targets](#hook_targets) | 不活性属性 | `?`, `return`, 末尾式(tail_expr)それぞれについてフックを行うかを切り替え(デフォルトは3種すべてにフック) |
| [tail_expr_idents](#tail_expr_idents) | 不活性属性 | 末尾式に来た時にフックを行うidentを指定(デフォルトでは `Err` ) |
| [ignore_tail_expr_idents](#ignore_tail_expr_idents) | 不活性属性 | フック対象であった場合でもフックを行わないidentを指定(デフォルトでは `Ok` ) |
| [result_types](#result_types) | 不活性属性 | `return` と末尾式にフックを行う関数の返り値型を指定(デフォルトは `Result` ) |
| [hook_in_macros](#hook_in_macros) | 不活性属性 | マクロ内にもフックを行うかを指定(デフォルトは `true` ) |
| [binding](#binding) | 不活性属性 | 指定したリテラル・式で置換されるメタ変数を作成 |

- マクロルートのメタ: 属性マクロ本体 `#[hooq(...)]` の `...` 部分に指定する属性(メタ)
- 不活性属性: 属性マクロ本体( `#[hooq]`, `#[hooq(...)]` )ではなく、その後に来るアイテムの随所に付与される属性。 `#[hooq::ATTR(...)]` のようなフォーマットで指定

すべての属性を付与してみたソースコードは以下のような感じです。

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/all_attr/src/main.rs}}
```

上記のソースコードでマクロが展開されると次のようになります。

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/all_attr/src/main.expanded.rs}}
```

## ルートメタ

hooq属性マクロ( `#[hooq]` )をアイテムに付与する際に、メタ部分で指定できる事項が2つ存在します。

| 名前 | 付与方法 |
|:----|:-------|
| flavor | `#[hooq(FLAVOR_NAME)]` or `#[hooq(flavor = "FLAVOR_NAME")]` |
| trait_use | `#[hooq(trait_use(TRAIT_PATH))]` |

解説は各項でします。

### flavor

`#[hooq(FLAVOR_NAME)]` または `#[hooq(flavor = "FLAVOR_NAME")]` というフォーマットでベースとなる設定をフレーバーから設定します。

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/attribute-flavor-1/src/main.rs}}
```

hooq.toml の内容は以下であるとします。

```toml
{{#include ../../../../../mdbook-source-code/attribute-flavor-1/hooq.toml}}
```

この時フレーバーの設定が読まれ展開後は以下のようになります。

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/attribute-flavor-1/src/main.expanded.rs:6:}}
```

例ではサブフレーバーも利用しています。サブフレーバーは、元となるフレーバーに上書きで設定を施したものを利用できる機能です。

サブフレーバーは文字列の場合 `.` か `::`、パスのような形で直接渡す場合 `::` で区切られた名前空間で表し、ネストさせる(つまり、サブフレーバーにさらにサブフレーバーを設ける)ことができます。

フレーバーのさらなる情報、特にhooq.tomlへの書き方等については[フレーバー](./flavors.md)のページを参照してください。

### trait_use

`#[hooq(trait_use(トレイトパス, ...))]` というフォーマットで指定します。 `trait_use` ではなく `trait_uses` でも構いません[^trait_use]。この時指定されたトレイトパスに対して `#[allow(unused)] use トレイトパス as _;` というuse文が、 `#[hooq(...)]` を付与したアイテムの前に出力されます。

[^trait_use]: 開発中に表記ゆれが起きてしまい両方残していますが、機能的には変わりません

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/trait_use/src/main.rs}}
```

展開後の `main` 関数とその上に出力されるuse文を抜粋すると次のようになっています。

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/trait_use/src/main.expanded.rs:20:25}}
```

例で示したように、 `trait_use` はフックするメソッドを呼び出すのに必要なトレイトを一緒に出力する目的で設けています。もちろんこの属性指定を利用せず直接use文を書いてしまっても構いませんが、例えば `#[cfg_attr(test, hooq(flavor = "test", trait_use(Trait)))]` のように限定的にhooqの利用をしたい場合などに記述を多少見やすくする効果が期待できます。

あるいは、hooq.tomlでも `trait_use` は(`trait_uses` で)指定できるため、 システム上一応設けたとも言えます。例えばanyhowフィーチャーでは [`with_context`](https://docs.rs/anyhow/latest/anyhow/trait.Context.html#tymethod.with_context) メソッドを挿入しますが、この時同時にuseが必要なトレイトである [`anyhow::Context`](https://docs.rs/anyhow/latest/anyhow/trait.Context.html) をuse文で導入しています[^context]。

[^context]: 置換モードを用いて書き方を工夫すれば実はトレイトのuseは不要だったりしますが、本マクロでは人間が良く書く直感的な記法の方を挿入する形を採用しています。

## method

## フックの付与をスキップする

### skip_all

### skip

## フック対象判定を制御する

### hook_targets

### tail_expr_idents

### ignore_tail_expr_idents

### result_types

## hook_in_macros

## binding

## flavor を利用した各属性の指定方法
