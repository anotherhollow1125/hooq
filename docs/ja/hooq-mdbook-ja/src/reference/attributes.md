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
{{#rustdoc_include ../../../../../mdbook-source-code/attribute-method/src/main.rs}}
```

上の例のように、「 `#[hooq]` マクロのすぐ下」・「関数内部」のどちらでも設定の変更を行うことができます。これは以降紹介する不活性属性で共通の性質です。 `main` 関数は以下のように展開されます。

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/attribute-method/src/main.expanded.rs:6:17}}
```

`#[hooq::method(...)]` には **挿入モード** と **置換モード** の2つのモードがあります。

- `.` (ドット)で始まる場合は挿入モードです。対象の式と `?` の間(あるいは `return` や末尾式なら単に式の末尾)にメソッドを挿入します。
- 上記以外は置換モードになり、与えられた式によって対象の式を置換します。対象の式は `$expr` メタ変数で得られるので、例えば `fn func<T, E>(_r: Result<T, E>) {}` などに対して `#[hooq::method(func($expr))]` のように書くことで関数で対象式をラップするといった記述が可能です。

その他、フックするメソッド内では `$line` や `$source` といった **メタ変数** を利用することが可能です。

ユーザーが特に何も指定しない場合、defaultフレーバーの設定値である以下のメソッドが挿入されます。

```rust
.inspect_err(|e| {
    let path = $path;
    let line = $line;
    let col = $col;
    let expr = ::hooq::summary!($source);

    ::std::eprintln!("[{path}:{line}:{col}] {e:?}\n{expr}");
})
```

モードやメタ変数に関する詳細は[メソッド](./method.md)と[メタ変数](./meta_vars.md)のページを参照してください。

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

skipは、末尾式がネストしてしまっており、そのままロギングをフックするとログが見辛くなってしまう場合等に有用です。

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/skip/src/main.rs::24}}
```

`#[hooq::skip]` がついている `func2` の方ではmatch式の外につく `inspect_err` がなくなっていることがわかります。

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/skip/src/main.expanded.rs:7:19}}
```

## フック対象判定を制御する

以下は各式に対して、フックするかしないかをカスタマイズするための不活性属性群になります。

| 名前 | 付与方法 | `...` の部分が取りうる値 |
|:----|:-------|:----------|
| hook_targets | `#[hooq::hook_targets(...)]` | `?` or `return` or `tail_expr` の中から複数 |
| tail_expr_idents | `#[hooq::tail_expr_idents(...)]` | `Err` など |
| ignore_tail_expr_idents | `#[hooq::ignore_tail_expr_idents(...)]` | `Ok` など |
| result_types | `#[hooq::result_types(...)]` | `Result` など |
| hook_in_macros | `#[hooq::hook_in_macros(...)]` | `true` or `false` |

設定値詳細は各項目にて示します。ここには、設定の適用優先順位を記します[^flowchart]。

[^flowchart]: フローチャートで示したかったのですがmdbook-mermaidがうまく機能しなかったため普通に箇条書きで示しています。

- `skip_all` が付与されている場合はフックしない
    - `skip` の場合は子スコープを除いた同スコープ内についてのみフックしない
- 対象式がマクロ呼び出し内部にあり、かつ `hook_in_macros` が `false` である場合はフックしない
- `?` へのフックの場合
    - `hook_targets` に `?` が含まれていればフックする
- `return` へのフックの場合
    - `hook_targets` に `return` が含まれていなければフックしない
    - 返り値の識別子が `tail_expr_idents` に含まれていればフックする
    - 関数の返り値型が `result_types` に含まれ、かつ返り値の識別子が `ignore_tail_expr_idents` に含まれない場合フックする
- 末尾式へのフックの場合
    - `hook_targets` に `tail_expr` が含まれていなければフックしない
    - 末尾式の識別子が `tail_expr_idents` に含まれていればフックする
    - 関数・クロージャが持つブロックの末尾式であり、かつその関数・クロージャの返り値型が `result_types` に含まれ、かつ返り値の識別子が `ignore_tail_expr_idents` に含まれない場合フックする

### hook_targets

`?` 演算子(Question Operator)、 `return`、 末尾式( `tail_expr` )の3つについて、それぞれフックするかしないかを指定できます。デフォルトでは3種類すべてフックされます。

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/hook_targets/src/main.rs::56}}
```

展開結果は次の通りとなります。

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/hook_targets/src/main.expanded.rs:10:31}}
```

今回は個別に書きましたが、 `#[hooq::hook_targets("?", "return")]` のように一つだけ抜く書き方等ももちろんできます。

### tail_expr_idents

`return` の返り値あるいは末尾式が指定した識別子である際は、関数の返り値型が [`result_types`](#result_types) に含まれるかに関わらずフックを行います。デフォルトは `Err` で、カンマ区切りで複数指定可能です。

<div class="warning">

識別子にはパス( `xxx::yyy::Zzz` )は認められず、単体( `Zzz` )である必要があります。

一致に関しては、パス中で最後の識別子(`xxx::yyy::Zzz` なら `Zzz`)で判別されます。
</div>

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/tail_expr_idents/src/main.rs}}
```

`tail_expr_idents` のおかげで、ブロックの返り値などに `Err` が含まれる場合、そこにもフックが行われるようになります。 `Err` 以外にこのような性質を持たせたい識別子がある際は `tail_expr_idents` に加えることで同様の挙動になります。

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/tail_expr_idents/src/main.expanded.rs:10:26}}
```

### ignore_tail_expr_idents

[`tail_expr_idents`](#tail_expr_idents) とは対照的に、 `return` の返り値あるいは末尾式が指定した識別である際は関数の返り値型が [`result_types`](#result_types) に含まれている場合でも **フックを行いません** 。デフォルトは `Ok` で、カンマ区切りで複数指定可能です。

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/ignore_tail_expr_idents/src/main.rs}}
```

上記例にあるように、 `ignore_tail_expr_idents` を利用せずとも、 `tail_expr_idents` への指定において頭に `!` (Exclamation Mark) を付けることでも同様の設定を行うことが可能です。展開結果は以下の通りとなります。

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/ignore_tail_expr_idents/src/main.expanded.rs:10:17}}
```

<div class="warning">
<b>両方に含まれる場合の挙動について</b>

同じ識別子が `tail_expr_idents` と `ignore_tail_expr_idents` の両方に含まれる場合、機構が単純なため **フックされてしまいます**。なるべく `!` (Exclamation Mark) を利用して `tail_expr_idents` 経由で設定した方が確実です。

<details><summary><code>Err</code>の例</summary>

`Err` はデフォルトで `tail_expr_idents` に含まれるので、 `ignore_tail_expr_idents` で指定してもフックされてしまいます。

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/ignore_tail_expr_idents-warning/src/main.rs}}
```

クロージャ `g` ではフックしてほしくないのにフックされていることが確認できます。

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/ignore_tail_expr_idents-warning/src/main.expanded.rs:7:14}}
```

</details>

</div>

### result_types

hooqマクロを付与した関数の返り値がフック対象であるかを判別するための識別子を設定する不活性属性です。関数の返り値型の識別子が `result_types` で指定した識別子と一致する時、 `return` や末尾式でフックを行うようになります。デフォルトは `Result` で、カンマ区切りで複数指定可能です。

<div class="warning">

[`tail_expr_idents`](#tail_expr_idents)等と同様に、識別子にはパス( `xxx::yyy::Zzz` )は認められず、単体( `Zzz` )である必要があります。

一致に関しては、パス中で最後の識別子(`xxx::yyy::Zzz` なら `Zzz`)で判別されます。
</div>

`Result` 型以外に独自で別な型を扱う際などに有用です。

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/result_types/src/main.rs::39}}
```

展開結果は次のようになります。

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/result_types/src/main.expanded.rs:10:23}}
```

### hook_in_macros

関数風マクロ呼び出し内に存在する対象の式に対し、フックを行うかを決定します。デフォルトは `true` でありマクロ呼び出し内までフックが行われます。

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/hook_in_macros/src/main.rs}}
```

展開結果は次の通りです。 `println!` マクロも展開されている点に注意してください。

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/hook_in_macros/src/main.expanded.rs:10:18}}
```

関数風マクロの引数部分はRustの文法に従った構文になっているとは限らず、フックを行う場合に解析に少しコストがかかるためオフにできる不活性属性を設けました。マクロの内側までフックする必要がない場合はこちらの設定を `false` にすることで多少コンパイル時間が短くなるかもしれません(多分)。

## binding

[メタ変数](./meta_vars.md) において、ユーザーが自由に式を保存できる **バインディング** 機能があります。同じ意味を表すいくつかの書き方があります。

| 付与方法 |備考|
|:-------|:--|
| `#[hooq::binding(xxx = ...)]` ||
| `#[hooq::var(xxx = ...)]` ||
| `#[hooq::xxx = ...]`| この方法で記述する場合 `xxx` は他不活性属性と名前衝突不可 |

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/attribute-binding/src/main.rs}}
```

展開結果は次のようになります。

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/attribute-binding/src/main.expanded.rs:10:34}}
```

[バインディング](./meta_vars.md#bindings) については当該ページの方も参照してください。

- [bindings](./meta_vars.md#bindings)

## flavor を利用した設定の部分適用

ここまで紹介してきた不活性属性による設定は、フレーバーを用いた部分適用が可能です。部分適用は `#[hooq::属性 = フレーバー名]` で行います。

また、すべての設定を不活性属性で上書きしたい場合は `#[hooq::flavor = フレーバー名]`、存在するユーザー定義のバインディングを上書きしたい場合は `#[hooq::bindings = フレーバー名]` という記法が使えます。

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/attribute-part-flavor/src/main.rs}}
```

hooq.tomlの内容は以下である時、

```toml
{{#include ../../../../../mdbook-source-code/attribute-part-flavor/hooq.toml}}
```

展開結果は次のようになります。

```rust
{{#rustdoc_include ../../../../../mdbook-source-code/attribute-part-flavor/src/main.expanded.rs:6:43}}
```