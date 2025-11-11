# フレーバーでプリセットを作成/利用する

[前のレッスン](./lesson-02-method.md) の最後に、 `Option` 型のメソッド `ok_or_else` が `Result` 型にはないためにフックをスキップするという話をしました。

そもそも `Result` と `Option` の両方にあるメソッドをフックすることはできないでしょうか...？ [anyhowクレート](https://docs.rs/anyhow/latest/anyhow/)が提供するメソッド [`Context::with_context`](https://docs.rs/anyhow/latest/anyhow/trait.Context.html#tymethod.with_context) がまさにうってつけです。 `Option` 型に対して `.with_context(...)` を呼ぶと、 `None` の時は `anyhow::Result` の `Err` バリアントへと変換を行ってくれます。

```rust
use anyhow::{Context, Result};
use hooq::hooq;

#[hooq]
#[hooq::method(.with_context(|| {
    let path = $path;
    let line = $line;
    let col = $col;
    let expr = $expr_str_short;

    format!("[{path}:{line}:{col}]
    {expr}")
}))]
fn display_name_by_mista(val: &toml::Value) -> Result<()> {
    let name = val.get("package")?.get("name")?.as_str()?;

    if name.contains("4") {
        return Err(anyhow::anyhow!(
            "name `{name}` contains '4'. Guido Mista disallow this."
        ));
    }

    println!("Mista「name: {name}」");

    Ok(())
}

#[hooq]
#[hooq::method(.with_context(|| {
    let path = $path;
    let line = $line;
    let col = $col;
    let expr = $expr_str_short;

    format!("[{path}:{line}:{col}]
    {expr}")
}))]
fn main() -> Result<()> {
    let path = std::env::args().nth(1).unwrap_or("Cargo.toml".to_string());

    let cargo_toml: toml::Value = toml::from_str(&std::fs::read_to_string(path)?)?;

    display_name_by_mista(&cargo_toml)?;

    Ok(())
}
```

エラーが発生するように実行すると、 `.with_context(...)` の分だけエラーがスタックされ、トレースが得られます。

```bash
Error: [src/main.rs:43:39]
    display_name_by_mista(&cargo_toml)?

Caused by:
    0: [src/main.rs:18:9]
           return Err(
       ...
       );
    1: name `contains_4` contains '4'. Guido Mista disallow this.
```

anyhowクレートはとても良く使われるクレートであり、そして `.with_context(...)` はここまでで示した通りとても便利なメソッドです。

そこで、このフックは頻出であろと考えhooqではanyhowに対し特別なプリセット... **anyhow フレーバー** を設けています。hooqではプリセットのことをわかりやすさのためフレーバー(flavor)と呼称しています。anyhowにとどまらず[log](https://docs.rs/log/latest/log/), [eyre](https://docs.rs/eyre/latest/eyre/)や[tracing](https://docs.rs/tracing/latest/tracing/)といったクレートに対してもフレーバーを用意をしています。

今まで `#[hooq]` と記載していた属性マクロ呼び出しを `#[hooq(フレーバー名)]` とすることでフレーバーを指定できます。

```rust
{{#rustdoc_include ../../../../../examples/examples/tutorial/tutorial_with_anyhow.rs}}
```

結果はフレーバーを使わないソースコードと同一です！

あらかじめ用意されているフレーバーについては[フレーバー](../reference/flavors.md)のページを参照してください。

## よく使う設定をフレーバーとして用意する

hooqクレート側で予め用意したフレーバーだけでなく、ユーザーが事前に用意したフレーバーを用いることも可能です。

フレーバーはクレートルート( `CARGO_MANIFEST_DIR` )に `hooq.toml` というファイルを設置することで可能です。以下は `hooq.toml` の設定例です。

```toml
{{#include ../../../../../examples/hooq.toml::22}}
```

詳細は[フレーバー](../reference/flavors.md)ページにて解説していますが、この時の設定項目の意味は次の通りです。

|設定項目|効果|
|:-----|:---|
| `method` | フックするメソッドを設定 |
| `hook_targets` | フック対象を3種から設定 |
| `tail_expr_idents` | 末尾式 / `return` にてこの設定項目の識別子(ident)である時、フックを行う |
| `result_types` | `#[hooq]` 対象がここに指定された型の関数である時、返り値はフック対象であるとみなされる |

`my_flavor.ok_or_else` というテーブルは、 `my_flavor` の **サブフレーバー** になります。設定項目を `my_flavor` から引き継ぎつつ、新たに設定を上書きすることが可能です。

設定可能なすべての項目と設定値による挙動(優先度等)については、[属性](../reference/attributes.md)と[フレーバー](../reference/flavors.md)のページをそれぞれ参照してください。

設定したフレーバーはソースコードから利用できるようになります。

```rust,ignore
{{#rustdoc_include ../../../../../examples/examples/flavor.rs}}
```

`#[hooq::method = フレーバー名]` という記法が出てきました。こちらはフレーバーの設定項目を部分適用するための機能です。その他の設定項目に関しても同様に部分適用が可能です。詳しくは[属性](../reference/attributes.md)の方を参照ください。

<div class="warning">
とても地味な注意なのですが、 `.$so_far` メタ変数による上書きは(フレーバーによって)最初に設定されたメソッドからの上書きを設定するものであるため、途中のアトリビュートからのみ可能になります。

言い換えると、 `#[hooq(フレーバー名)]` で指定したフレーバーのmethodには `$so_far` を含めることはできず、 `#[hooq(my_flavor::ok_or_else)]` はフレーバーの指定方法としては正しいのですがコンパイルエラーになってしまいます。

`#[hooq::method = my_flavor::ok_or_else]` は上書きに当たるため `.$so_far` の利用が可能です。
</div>

## まとめ

3つのレッスンを通してhooqの基本的な使い方を紹介してきました。

レッスンでは触れられなかった細かい仕組みや仕様がまだまだあるので、気になった機能の紹介はぜひ個別のドキュメントを参照いただきたいです。

- [リファレンス](../reference/index.md): 各機能に関しての詳細解説をしています。
- [レシピ・アイデア集](../recipe/index.md): あらかじめ用意されているフレーバーのソースコード利用例や、hooqのユースケース、隠し機能などを載せています。