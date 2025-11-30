# features

hooqクレートにもいくつかのfeaturesが存在します。本ページに表としてまとめておきます。

| feature名 | default | 説明 |
|:---------|:--------:|:----|
| default | o | デフォルトfeature。 `consume-question` 以外を含む |
| full | x | すべてのfeatureを含むfeature |
| anyhow | o | anyhowフレーバーを提供するfeature |
| eyre | o | eyreフレーバーを提供するfeature |
| log | o | logフレーバーを提供するfeature |
| tracing | o | tracingフレーバーを提供するfeature |
| consume-question | x | `!` (Exclamation Mark)による `?` 演算子 (Question Operator) の削除を行えるようになるfeature |

`consume-question` のみがデフォルトから外れた機能であり、フレーバーを提供するためのfeatureはデフォルトで含まれています。
