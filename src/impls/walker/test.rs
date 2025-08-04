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

    macrotest::expand("tests/walker/stmt/local.rs");
    macrotest::expand("tests/walker/stmt/item.rs");
    macrotest::expand("tests/walker/stmt/expr.rs");

    mask_project_root("tests/walker/stmt", Mask);
}

#[test]
fn test_walk_item() {
    mask_project_root("tests/walker/item", UnMask);

    macrotest::expand("tests/walker/item/const.rs");
    macrotest::expand("tests/walker/item/fn.rs");
    macrotest::expand("tests/walker/item/impl.rs");
    macrotest::expand("tests/walker/item/mod_.rs");
    macrotest::expand("tests/walker/item/static.rs");
    macrotest::expand("tests/walker/item/trait.rs");

    mask_project_root("tests/walker/item", Mask);
}

#[test]
fn test_walk_expr() {
    mask_project_root("tests/walker/expr", UnMask);

    macrotest::expand("tests/walker/expr/try.rs");
    macrotest::expand("tests/walker/expr/return.rs");
    macrotest::expand("tests/walker/expr/array.rs");
    macrotest::expand("tests/walker/expr/assign.rs");
    macrotest::expand("tests/walker/expr/async.rs");
    macrotest::expand("tests/walker/expr/await.rs");
    macrotest::expand("tests/walker/expr/binary.rs");
    macrotest::expand("tests/walker/expr/block.rs");
    macrotest::expand("tests/walker/expr/break.rs");
    macrotest::expand("tests/walker/expr/call.rs");
    macrotest::expand("tests/walker/expr/cast.rs");
    macrotest::expand("tests/walker/expr/closure.rs");
    macrotest::expand("tests/walker/expr/const.rs");
    macrotest::expand("tests/walker/expr/field.rs");
    macrotest::expand("tests/walker/expr/for_loop.rs");
    macrotest::expand("tests/walker/expr/group.rs");
    macrotest::expand("tests/walker/expr/if.rs");
    macrotest::expand("tests/walker/expr/index.rs");
    macrotest::expand("tests/walker/expr/let.rs");
    macrotest::expand("tests/walker/expr/loop.rs");
    macrotest::expand("tests/walker/expr/match.rs");
    macrotest::expand("tests/walker/expr/method_call.rs");
    macrotest::expand("tests/walker/expr/paren.rs");
    macrotest::expand("tests/walker/expr/range.rs");
    macrotest::expand("tests/walker/expr/repeat.rs");
    macrotest::expand("tests/walker/expr/struct.rs");
    macrotest::expand("tests/walker/expr/tuple.rs");
    macrotest::expand("tests/walker/expr/unary.rs");
    macrotest::expand("tests/walker/expr/unsafe.rs");
    macrotest::expand("tests/walker/expr/while.rs");

    mask_project_root("tests/walker/expr", Mask);
}
