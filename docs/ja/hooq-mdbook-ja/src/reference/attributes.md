# 属性

hooqマクロの挙動は、 `#[hooq(ルートメタ)]` におけるルートメタおよび、途中に挿入される不活性属性 ( `#[hooq::属性(...)]` ) によって変更できます。

本ページでは属性による設定可能項目および設定値のデフォルトを解説します。

**クイックリファレンス**

| 名前 | 種別 | 説明 |
|:----|:----|:----|
| [flavor](#flavor) | マクロルートのメタ | 指定したフレーバーの設定を適用する |
| [trait_use](#trait_use) | マクロルートのメタ | 指定したパス( `XXX` )について `use XXX as _;` をアイテムの前に挿入する |
| [method](#method) | 不活性属性 | 挿入/置換するメソッド(置換の場合は式)を設定する |
| [skip_all](#skip_all) / [skip](#skip) | 不活性属性 | 本属性を付与した式へのフックは行わないようになる |
| [hook_targets](#hook_targets) | 不活性属性 | `?`, `return`, 末尾式(tail_expr)それぞれについてフックを行うかを切り替え(デフォルトは3種すべてにフック) |
| [tail_expr_idents](#tail_expr_idents) | 不活性属性 | 末尾式に来た時にフックを行うidentを指定(デフォルトでは `Err` ) |
| [ignore_tail_expr_idents](#ignore_tail_expr_idents) | 不活性属性 | フック対象であった場合でもフックを行わないidentを指定(デフォルトでは `Ok` ) |
| [result_types](#result_types) | 不活性属性 | `return` と末尾式にフックを行う関数の返り値型を指定(デフォルトは `Result` ) |
| [hook_in_macros](#hook_in_macros) | 不活性属性 | マクロ内にもフックを行うかを指定(デフォルトは `true` ) |
| [binding](#binding) | 不活性属性 | 指定したリテラル・式で置換されるメタ変数を作成 |

- マクロルートのメタ: 属性マクロ本体 `#[hooq(...)]` の `...` 部分に指定する属性(メタ)
- 不活性属性: 属性マクロ本体( `#[hooq]`, `#[hooq(...)]` )ではなく、その後に来るアイテムの随所に付与される属性。 `#[hooq::属性(...)]` のようなフォーマットで指定

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
| trait_use | `#[hooq(trait_use(TRAIT_PATH, ...))]` or `#[hooq(trait_uses(TRAIT_PATH, ...))]` |

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

例ではサブフレーバーも利用しています。サブフレーバーは、元となるフレーバーに上書きで設定を施したフレーバーです。

サブフレーバーは文字列の場合 `.` か `::`、パスのような形で直接渡す場合 `::` で区切られた名前空間で表し、ネストさせる(つまり、サブフレーバーにさらにサブフレーバーを設ける)ことができます。

フレーバーのさらなる情報、特にanyhowなどのhooqが予め用意しているフレーバーや、hooq.tomlへの書き方等については[フレーバー](./flavors.md)のページを参照してください。

- [フレーバー](./flavors.md)

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

例で示したように、 `trait_use` はフックするメソッドを呼び出すのに必要なトレイトを一緒に出力する目的で設けています。もちろんこの属性指定を利用せず直接use文を書いてしまっても構いませんが、例えば `#[cfg_attr(test, hooq(flavor = "test", trait_use(Trait)))]` のようにコンパイル条件付きでhooqの利用をしたい場合などに、記述を多少見やすくする効果が期待できます。

あるいは、hooq.tomlでも `trait_use` は(`trait_uses` フィールドで)指定できるため、 システム上一応属性でも設定可能にしたとも言えます。例えばanyhowフィーチャーでは [`with_context`](https://docs.rs/anyhow/latest/anyhow/trait.Context.html#tymethod.with_context) メソッドを挿入しますが、この時同時に必要なトレイトである [`anyhow::Context`](https://docs.rs/anyhow/latest/anyhow/trait.Context.html) をuse文で導入しています[^context]。

[^context]: 置換モードを用いて書き方を工夫すれば実はトレイトのuseは不要だったりしますが、hooqマクロでは人間が良く書く直感的な記法の方を挿入する形を採用しています。

## method

| 名前 | 付与方法 |
|:----|:-------|
| method | `#[hooq::method(...)]` |

`#[hooq::method(.method_name())]` のようなフォーマットで、フックするメソッドを設定できる不活性属性です。

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/method/src/main.rs}}
```

上の例のように、「 `#[hooq]` マクロのすぐ下」・「関数内部」のどちらでも設定の変更を行うことができます。これは以降紹介する不活性属性で共通の性質です。 `main` 関数は以下のように展開されます。

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/method/src/main.expanded.rs:6:17}}
```

`#[hooq::method(...)]` には **挿入モード** と **置換モード** の2つのモードがあります。

- `.` (ドット)で始まる場合は挿入モードです。対象の式と `?` の間(あるいは `return` や末尾式なら単に式の末尾)にメソッドを挿入します。
- 上記以外は置換モードになり、与えられた式によって対象の式を置換します。対象の式は `$expr` メタ変数で得られるので、例えば `fn func<T, E>(_r: Result<T, E>) {}` などに対して `#[hooq::method(func($expr))]` のように書くことで関数で対象式をラップするといった記述が可能です。

その他、フックするメソッド内では `$line` や `$source` といった **メタ変数** を利用することが可能です。

これらモードやメタ変数に関する詳細は[メソッド](./method.md)と[メタ変数](./meta_vars.md)のページを参照してください。

- [メソッド](./method.md)
- [メタ変数](./meta_vars.md)

## フックの付与をスキップする

| 名前 | 付与方法 |
|:----|:-------|
| skip_all | `#[hooq::skip_all]` |
| skip | `#[hooq::skip]` |

### skip_all

hooqは(デフォルトの設定では)かなり貪欲にメソッドを `?` 等にフックしてきます。「この `?` にはフックしたくない！」といった要望は自然に起きると思います。そのような場合はフック付与を行いたくない式・文に `#[hooq::skip_all]` を付けてください。

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/skip_all/src/main.rs}}
```

付与した式とその内部にはフックされなくなります。

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/skip_all/src/main.expanded.rs:7:15}}
```

### skip

`#[hooq::skip]` は滅多に利用することはないであろう不活性属性になりますが、一応用意した `#[hooq::skip_all]` の亜種になります。

skip_allでは付与対象全体でフックがスキップされましたが、skipでは **付与した(親)スコープでのみ** フックのスキップが起きます。

skipは、末尾式がネストしてしまっており、そのままロギングをフックするとログが見辛くなってしまう場合に有用です。

## フック対象判定を制御する

### hook_targets

### tail_expr_idents

### ignore_tail_expr_idents

### result_types

## hook_in_macros

## binding

## flavor を利用した各属性の指定方法
