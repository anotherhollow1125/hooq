# 属性

hooqマクロの挙動は、 `#[hooq(ルートメタ)]` におけるルートメタおよび、途中に挿入される不活性属性 ( `#[hooq::ATTR(...)]` ) によって変更できます。

本ページでは属性による設定可能項目および設定値のデフォルトを解説します。

**クイックリファレンス**

| 名前 | 種別 | 説明 |
|:----|:----|:----|
| [flavor](#flavor) | マクロルートのメタ | 指定したフレーバーの設定を適用する |
| [trait_use](#trait_use) | マクロルートのメタ | 指定したパス( `XXX` )について `use XXX as _;` をアイテムの前に挿入する |
| [method](#method) | 不活性属性 | 挿入/置換するメソッド(置換の場合は式)を設定する |
| [skip_all](#skip_all) / [skip](#skip) | 不活性属性 | 本属性を付与した式へのフックは行わないようになる |
| [hook_targets](#hook_targets) | 不活性属性 | `?`, `return`, 末尾式(tail_expr)それぞれについてフックを行うかを切り替え(デフォルトはすべて) |
| [tail_expr_idents](#tail_expr_idents) | 不活性属性 | 末尾式に来た時にフックを行うidentを指定(デフォルトでは `Err` ) |
| [ignore_tail_expr_idents](#ignore_tail_expr_idents) | 不活性属性 | フック対象であった場合でもフックを行わないidentを指定(デフォルトでは `Ok` ) |
| [result_types](#result_types) | 不活性属性 | `return` と末尾式にフックを行う関数の返り値型を指定(デフォルトは `Result` ) |
| [hook_in_macros](#hook_in_macros) | 不活性属性 | マクロ内にもフックを行うかを指定(デフォルトは `true` ) |
| [binding](#binding) | 不活性属性 | 指定したリテラル・式で置換されるメタ変数を作成 |

- マクロルートのメタ: 属性マクロ本体 `#[hooq(...)]` の `...` 部分に指定する属性(メタ)
- 不活性属性: 属性マクロ本体( `#[hooq]`, `#[hooq(...)]` )ではなく、その後に来るアイテムの随所に付与される属性。 `#[hooq::ATTR(...)]` のようなフォーマットで指定

## ルートメタ

### flavor

### trait_use

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
