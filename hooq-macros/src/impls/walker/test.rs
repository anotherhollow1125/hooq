use glob::glob;

// macrotest は別なディレクトリを作ってそこで検証するために $file に対する出力の調整が難しい
// よってテスト前後に無理やり修正することで環境差をなくす

enum MaskMode {
    Mask,
    UnMask,
}

use MaskMode::*;

// *.expanded.rs ファイルのプロジェクトルートをマスクする
// /home/user_name/../hooq <-> <hooq_root>
fn mask_project_root(target_path: &str, mode: MaskMode) {
    // *.expanded.rs ファイルのリストを取得
    let files = glob(format!("{target_path}/**/*.expanded.rs").as_str())
        .expect("Failed to read glob pattern");

    // プロジェクトルートを取得
    let project_root = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is not set");

    // プロジェクトルートをマスクする
    for entry in files {
        let Ok(entry) = entry else {
            continue;
        };

        let content = std::fs::read_to_string(&entry).expect("Failed to read file");
        let processed_content = match mode {
            Mask => content.replace(&project_root, "<hooq_root>"),
            UnMask => content.replace("<hooq_root>", &project_root),
        };
        std::fs::write(&entry, processed_content).expect("Failed to write file");
    }
}

#[test]
fn test_walk_stmt() {
    mask_project_root("tests/walker/stmt", UnMask);

    macrotest::expand_args("tests/walker/stmt/local.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/stmt/item.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/stmt/expr.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/stmt/macro.rs", &["--ugly"]);

    mask_project_root("tests/walker/stmt", Mask);
}

#[test]
fn test_walk_item() {
    mask_project_root("tests/walker/item", UnMask);

    macrotest::expand_args("tests/walker/item/const.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/item/fn.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/item/impl.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/item/macro.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/item/mod_.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/item/static.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/item/trait.rs", &["--ugly"]);

    mask_project_root("tests/walker/item", Mask);
}

#[test]
fn test_walk_expr() {
    mask_project_root("tests/walker/expr", UnMask);

    macrotest::expand_args("tests/walker/expr/try.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/expr/return.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/expr/array.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/expr/assign.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/expr/async.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/expr/await.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/expr/binary.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/expr/block.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/expr/break.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/expr/call.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/expr/cast.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/expr/closure.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/expr/const.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/expr/field.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/expr/for_loop.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/expr/group.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/expr/if.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/expr/index.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/expr/let.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/expr/loop.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/expr/macro.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/expr/match.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/expr/method_call.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/expr/paren.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/expr/range.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/expr/repeat.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/expr/struct.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/expr/tuple.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/expr/unary.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/expr/unsafe.rs", &["--ugly"]);
    macrotest::expand_args("tests/walker/expr/while.rs", &["--ugly"]);

    mask_project_root("tests/walker/expr", Mask);
}
