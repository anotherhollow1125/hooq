use glob::glob;

// macrotest は別なディレクトリを作ってそこで検証するために $file に対する出力の調整が難しい
// よってテスト前後に無理やり修正することで環境差をなくす

pub enum MaskMode {
    Mask,
    UnMask,
}

use MaskMode::*;

// *.expanded.rs ファイルのプロジェクトルートをマスクする
// /home/user_name/../hooq <-> <hooq_root>
pub fn mask_project_root(target_path: &str, mode: MaskMode) {
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
