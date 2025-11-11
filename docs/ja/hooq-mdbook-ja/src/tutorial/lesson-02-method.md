# `#[hooq::method(...)]` でフックをカスタム

フックされるメソッドはユーザーが設定することが可能です。

プロジェクトのCargo.tomlの `name` を表示するプログラム例で説明します。tomlファイルを読めるようにしたいためtomlクレートを導入します。

```bash
cargo add toml
```

ひとまず指定されたパスのファイルをtomlファイルとして読み取る機構を作ります。

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args().nth(1).unwrap_or("Cargo.toml".to_string());

    let _cargo_toml: toml::Value = toml::from_str(&std::fs::read_to_string(path)?)?;

    // snip

    Ok(())
}
```

そのままhooqマクロを適用しても良いですが、今回はフックをカスタムしてみるというお題なのでやってみましょう。エラーなくうまくいった場合もわかるように、 [`inspect`](https://doc.rust-lang.org/std/result/enum.Result.html#method.inspect) メソッドも追加してみます。

フックするメソッドを指定したい時は、 `#[hooq::method(...)]` 属性を使います。現在設定済みのフックメソッドを表す `.$so_far` メタ変数を使うことで、追加分として `inspect` メソッドを入れることができます。

```rust
use hooq::hooq;

#[hooq]
#[hooq::method(.$so_far.inspect(|_| {
    println!("Success: `{}` @ Line {}: Col: {}", $expr_str_short_oneline, $line, $col);
}))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args().nth(1).unwrap_or("Cargo.toml".to_string());

    let _cargo_toml: toml::Value = toml::from_str(&std::fs::read_to_string(path)?)?;

    // snip

    Ok(())
}
```

失敗の場合いままで通りログが出ます。

```bash
[src/main.rs:10:81] Os { code: 2, kind: NotFound, message: "No such file or directory" }
    std::fs::read_to_string(path)?
Error: Os { code: 2, kind: NotFound, message: "No such file or directory" }
```

成功の場合もログを出力するようになりました！

```bash
Success: `std::fs::read_to_string(path)?` @ Line 10: Col: 81
Success: `... )?` @ Line 10: Col: 83
```

次は名前を表示する機構を作りましょう。tomlから目的のフィールドを `toml::Value::get` を使うことで抽出していきます。 `toml::Value::get` は `Option` を返します。[`ok_or_else`](https://doc.rust-lang.org/std/option/enum.Option.html#method.ok_or_else) を使うことで、 `Option` 型を `Result` 型に変換することが可能なので、今回はそのようにします。

```rust
use hooq::hooq;

fn display_name(val: &toml::Value) -> Result<(), String> {
    let name = val
        .get("package")
        .ok_or_else(|| format!("get package [Line: {}]", line!()))?
        .get("name")
        .ok_or_else(|| format!("get name [Line: {}]", line!()))?
        .as_str()
        .ok_or_else(|| format!("as_str [Line: {}]", line!()))?;

    println!("name: {name}");

    Ok(())
}

#[hooq]
#[hooq::method(.$so_far.inspect(|_| {
    println!("Success: `{}` @ Line {}: Col: {}", $expr_str_short_oneline, $line, $col);
}))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args().nth(1).unwrap_or("Cargo.toml".to_string());

    let cargo_toml: toml::Value = toml::from_str(&std::fs::read_to_string(path)?)?;

    display_name(&cargo_toml)?;

    Ok(())
}
```

...このような恣意的な例を持ち出して私が何を言いたいか、もうお分かりですね？ `.ok_or_else(...)` はボイラープレートです！hooqクレートを使えばこの記述は簡略化できます！

```rust
{{#rustdoc_include ../../../../../examples/examples/tutorial/tutorial.rs:3:17}}
```

今回、何番目の `?` 演算子であるかを示す `$nth` メタ変数も入れてみました。エラーになるように実行させると次のようになります。

```bash
Success: `std::fs::read_to_string(path)?` @ Line 26: Col: 80
Success: `... )?` @ Line 26: Col: 82
[src/main.rs:28:30] "... .get(\"name\")? [Line: 12, 2nd ?]"
    display_name(&cargo_toml)?
Error: "... .get(\"name\")? [Line: 12, 2nd ?]"
```

`package` フィールドは要れたが `name` フィールドがないtomlを読ませました。想定通り、2番目の `?` でエラーになったことが示されています。

## `#[hooq::skip_all]` でフックをスキップ

「4は `404` 等を連想させて不吉[^mista]だからタイトルに4が入っているかチェックするぜ！」と言われたのでバリデーションを付けることになりました。こんな感じでしょうか？

[^mista]: でもこれで行くとサーバーサイドエンジニアなどには `5` の方が不吉そうですね。

```rust,ignore
#[hooq]
#[hooq::method(.ok_or_else(|| {
    format!("{} [Line: {}, {}]",
        $expr_str_short_oneline,
        $line,
        $nth
    )
}))]
fn display_name_by_mista(val: &toml::Value) -> Result<(), String> {
    let name = val.get("package")?.get("name")?.as_str()?;

    if name.contains("4") {
        return Err(format!(
            "name `{name}` contains '4'. Guido Mista disallow this."
        ));
    }

    println!("Mista「name: {name}」");

    Ok(())
}
```

hooqは `return Err(...);` にもメソッドをフックすることを試みるため、 `Result` 型に `ok_or_else` メソッドが存在しないというコンパイルエラーになってしまいます[^error]。

[^error]: hooqマクロのSpan設定の影響でエラーの補足がおかしくなってしまっていますね。もう少しマシなSpanに直したいと思います。

```bash
$ cargo build
   Compiling project v0.1.0 (/path/to/project)
error[E0599]: no method named `ok_or_else` found for enum `Result` in the current scope
  --> src/main.rs:31:9
   |
31 |         return Err(format!(
   |         ^^^^^^
   |
help: there is a method `or_else` with a similar name
   |
31 -         return Err(format!(
31 +         or_else Err(format!(
   |

For more information about this error, try `rustc --explain E0599`.
error: could not compile `playground` (bin "playground") due to 1 previous error
```

ここは今回はフックしたくない場所です。そのような場合は `#[hooq::skip_all]` を付与することで、フックさせないようにできます！

```rust
{{#rustdoc_include ../../../../../examples/examples/tutorial/tutorial.rs:19:40}}
```

<details><summary>完成したプログラム全体</summary>

```rust
{{#rustdoc_include ../../../../../examples/examples/tutorial/tutorial.rs}}
```
</details>

無事にコンパイルが通り、名前に4が入っている時はエラーとなります。

```bash
Success: `std::fs::read_to_string(path)?` @ Line 49: Col: 80
Success: `... )?` @ Line 49: Col: 82
name: contains_4
Success: `display_name(&cargo_toml)?` @ Line 51: Col: 30
[src/main.rs:52:39] "name `contains_4` contains '4'. Guido Mista disallow this."
    display_name_by_mista(&cargo_toml)?
Error: "name `contains_4` contains '4'. Guido Mista disallow this."
```

`#[hooq::skip_all]` の他にも、途中でhooqの挙動を変更するための属性はいくつかあります。  `#[hooq::method(...)]` もその一つで、関数の冒頭だけではなく、個別の式や文に付与することも可能です。詳細が知りたい方は[属性](../reference/attributes.md)を参照してください。

ここまでカスタムフックを設定する方法を解説しました。しかし関数ごとに毎回 `#[hooq::method(...)]` で目当てのメソッドを設定するのはちょっと面倒ですよね？

[次のレッスン](./lesson-03-flavor.md)ではあらかじめカスタムフックを設定しておいたり、hooqクレート自体が用意している便利な設定を適用するための **フレーバー機能** について解説したいと思います。
