# コントリビューティング

改善点等あればぜひイシューやPRを投げてほしいです！本ページではコントリビューティングに必要であろうhooqの開発フロー周りやスタンス等を解説します。

## スナップショットテストについて

hooqはマクロクレートのため、相当数のスナップショットを残しています。スナップショット取得方法は主に3つです。

- [macrotest](https://docs.rs/macrotest/latest/macrotest/) クレート
    - マクロ展開後の状態を得るために使用
    - 大部分のスナップショットはこちらで取得しています
- [insta](https://docs.rs/insta/latest/insta/) クレート
    - mdBook執筆において、プログラムの標準出力/標準エラー出力を保存するために使用
- その他、独自のスナップショット
    - `hooq.toml` ファイルが必要な一部テストで実行
    - `cargo expand` の結果をそのまま保存
    - macrotestではうまく `hooq.toml` を扱えなかったため

テストの実行方法はシンプルです。プロジェクトルートで `cargo test` を打つことでワークスペースのテストがすべて実行されます。

```bash
cargo test
```

もしスナップショットに差分が出た場合エラーになります。その差分が意図通りのものである場合、以下の対応をして再実行してください。

- macrotest スナップショットおよび独自スナップショットの場合: 元々あるスナップショットファイルを削除してください。
- insta スナップショットの場合: `cargo insta review` コマンドで差分をAcceptしてください。
    - CLIツール `cargo-insta` が必要です。 `cargo install cargo-insta` で導入してください。

## CIについて

CIを利用してコードベースが乱れていないかを確認しています。

- [rust.yml](https://github.com/anotherhollow1125/hooq/blob/main/.github/workflows/rust.yml)

CIではテスト以外に以下を調べています。

- [cargo +nightly fmt](https://doc.rust-lang.org/cargo/commands/cargo-fmt.html)
    - `imports_granularity = "Module"` および `group_imports = "StdExternalCrate"` のルールを適用するため、nightlyを利用しています。
    - nightlyのバージョンについては [rust.yml](https://github.com/anotherhollow1125/hooq/blob/main/.github/workflows/rust.yml) を確認してください。
- [cargo sort --workspace](https://crates.io/crates/cargo-sort)
    - Cargo.toml 記載のクレート順を一意に保つために利用しています。
- [cargo clippy](https://github.com/rust-lang/rust-clippy)

## ドキュメントの更新および `sync.rs` コマンドについて

hooqは公開クレート数が3つでさらに英語版と日本語版のドキュメントを用意しています。またhooqの最新バージョンがREADMEに掲載されるようになっています。

これらドキュメントの状態を適切に守るため、 [`.github/scripts/sync.rs`](https://github.com/anotherhollow1125/hooq/blob/main/.github/scripts/sync.rs) というCargo Scriptを用意し、手元とCIで実行するようにしています。

このスクリプトでは以下2つのファイルを元に、各クレート・GitHubのREADME.mdを生成しています。

- 英語版: [docs/_readme_root.md.template](https://github.com/anotherhollow1125/hooq/blob/main/docs/_readme_root.md.template)
- 日本語版: [docs/ja/_readme_root.md.template](https://github.com/anotherhollow1125/hooq/blob/main/docs/ja/_readme_root.md.template)

そのため例えばタイポ等を見つけてREADME.mdを修正する場合は、上記のテンプレートを修正したのちに `sync.rs` コマンドを走らせて更新を行ってほしいです。

リポジトリルートにて以下のように実行することで更新されます。

```bash
./.github/scripts/sync.rs
```

## 言語(英語/日本語)について

プロジェクトオーナー( [anotherhollow1125](https://github.com/anotherhollow1125), 以降、「私」と呼称 )が日本人のため、コードベースに存在するコメントは基本的に日本語になっています。私は日本語こそコメントを記述するのに適した言語と考えているためこの方針は変えません。英語圏の方には申し訳ないです。

イシュー・PRについては英語・日本語両方OKです！ご自身の好きな言語で投げていただければ幸いです。

英語のドキュメントの大部分は生成AIに翻訳してもらったものになっています。そのためもし英語版ドキュメントにおいて用語の使い方やニュアンスに違和感がある場合はぜひイシュー・PRを出してほしいです！

## 生成AI利用のスタンスについて

昨今、バイブコーディングによりすべてのプログラムを生成AIに作成してもらう方法が流行っていますが、私はこの利用方法を疑問視しているため一切行っていません。100%生成AIにより作成されたPRは、軽い修正等を除きリジェクトされる可能性が高いことに留意してください。

しかしながら一切生成AIを使ってないわけではありません。hooqプロジェクトもかなり生成AIのお世話になっています。

私は主に以下の作業において生成AIを利用しています。

- [GitHub Copilot](https://github.com/features/copilot) によるコード**補完**
    - 前述の通り、ゼロから自然言語で生成AIにコーディング依頼をするような使い方はしていません
    - しかし、自身でコーディング中にCopilotに補完してもらう利用方法は多用しています。
- GitHub Copilot によるGitHub上でのレビュー
    - 不自然なコーディングになっていないかを調べるため、こちらはPRを出すたびに行ってきました。
- GitHub Copilot Chat / ChatGPTにアドバイスをもらう
    - プロジェクト全般を進めるにあたり生成AIに相談に乗ってもらっています
- GitHub Copilot Chat による日本語 → 英語の翻訳・ドキュメント生成
- [DeepWiki](https://deepwiki.com/)の利用
    - 存在するドキュメントをベースにし作成しました
    - <https://deepwiki.com/anotherhollow1125/hooq>
    - ハルシネーションが多少含まれているので参照される際はご注意ください

一方で以下の作業は手作業を重視しています。

- 開発主導
    - GitHub Copilotによる補完はなるべく小さい範囲で行い、すべてのコードベースを把握しながらコーディングしています。
- 元となる日本語ドキュメントの作成
    - コード例含み自分の言葉で記述することを重視しています。
    - ただし前述の通り、DeepWiki等生成AIを利用したドキュメンテーションも一応用意しています。
