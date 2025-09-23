use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{LazyLock, Mutex};

use proc_macro2::TokenStream;
use syn::{Expr, Path, parse_quote};

use crate::impls::flavor::toml_load::HooqToml;
use crate::impls::inert_attr::context::HookTargetSwitch;

pub mod r#macro;
mod presets;
mod toml_load;

#[derive(Debug, Clone)]
pub struct Flavor {
    pub trait_uses: Vec<Path>,
    pub method: TokenStream,
    pub hook_targets: HookTargetSwitch,
    pub tail_expr_idents: Vec<String>,
    pub ignore_tail_expr_idents: Vec<String>,
    pub result_types: Vec<String>,
    pub hook_in_macros: bool,
    pub bindings: HashMap<String, Rc<Expr>>,
    pub sub_flavors: HashMap<String, Flavor>,
}

impl Default for Flavor {
    fn default() -> Self {
        Self {
            trait_uses: Vec::new(),
            method: default_method(),
            hook_targets: HookTargetSwitch {
                question: true,
                return_: true,
                tail_expr: true,
            },
            tail_expr_idents: vec!["Err".to_string()],
            ignore_tail_expr_idents: vec!["Ok".to_string()],
            result_types: vec!["Result".to_string()],
            hook_in_macros: true,
            bindings: HashMap::new(),
            sub_flavors: HashMap::new(),
        }
    }
}

fn default_method() -> TokenStream {
    // NOTE:
    // $path や $line は eprintln! に直接埋め込みたいところだが、
    // CI側のテストの関係でこのようになっている
    // (恨むならeprintln!の仕様を恨んでください)

    parse_quote! {
        .inspect_err(|e| {
            let path = $path;
            let line = $line;

            ::std::eprintln!("[{path}:L{line}] {e:?}");
        })
    }
}

#[derive(Debug)]
pub struct FlavorStore {
    flavors: HashMap<String, Flavor>,
}

// NOTE:
// 本当はパース後のFlavorsをグローバルに保持したいが、OnceLockはSync制約があるため
// TokenStreamを使っているFlavorsはそのままでは保持できない
// そのため変換確認だけ行った状態でHooqTomlをグローバルに保持し、
// 実際にFlavorsが必要な時にはunwrapを許可することにした
#[derive(Debug, Clone)]
pub struct CheckedHooqToml {
    pub inner: HooqToml,
}

pub struct TomlStore {
    inner: Mutex<HashMap<String, CheckedHooqToml>>,
}

impl TomlStore {
    // NOTE:
    // 異なるCargoプロジェクトを同じタイミングでVSCodeで開いていた時に
    // 違うTomlStoreの内容が違うプロジェクトに供給されている事象が見られた
    //
    // 効果がどれぐらいあるかは懐疑的であるが、少しでも軽減すべく
    // プロジェクトごとに保存領域を分けるようにした
    pub fn set(&self, checked_hooq_toml: CheckedHooqToml) {
        let key = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());

        self.inner.lock().unwrap().insert(key, checked_hooq_toml);
    }

    fn get(&self) -> Option<CheckedHooqToml> {
        let key = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());

        self.inner.lock().unwrap().get(&key).cloned()
    }
}

pub static LOADED_HOOQ_TOML: LazyLock<TomlStore> = LazyLock::new(|| TomlStore {
    inner: Mutex::new(HashMap::new()),
});

impl FlavorStore {
    fn new() -> Self {
        let flavors = presets::preset_flavors();

        Self { flavors }
    }

    pub fn with_hooq_toml() -> Self {
        let mut flavors = Self::new();

        // toml_load()! 経由でHooqTomlがロードされていれば読み込む
        if let Some(hooq_toml) = LOADED_HOOQ_TOML.get() {
            // 変換が成功することはCheckedHooqTomlの生成時に確認済み
            toml_load::apply::update_flavors(&mut flavors.flavors, hooq_toml.inner).unwrap();
        }

        flavors
    }

    pub fn get_flavor(&self, path: &[String]) -> Option<Flavor> {
        let mut path = path.iter();
        let mut current: &Flavor = self.flavors.get(path.next()?)?;

        for name in path {
            current = current.sub_flavors.get(name)?;
        }

        Some(current.clone())
    }

    pub fn all_flavor_names(&self) -> Vec<String> {
        fn collect_names(flavor: &Flavor, prefix: &str, names: &mut Vec<String>) {
            let current_name = format!("{prefix}::");

            for (sub_name, sub_flavor) in &flavor.sub_flavors {
                let full_name = format!("{current_name}{sub_name}");
                names.push(full_name.clone());
                collect_names(sub_flavor, &full_name, names);
            }
        }

        let mut names = Vec::new();

        for (name, flavor) in &self.flavors {
            names.push(name.clone());
            collect_names(flavor, name, &mut names);
        }

        names
    }
}

impl TryFrom<HooqToml> for FlavorStore {
    type Error = String;

    fn try_from(value: HooqToml) -> Result<Self, Self::Error> {
        let mut flavors = Self::new();

        toml_load::apply::update_flavors(&mut flavors.flavors, value)?;

        Ok(flavors)
    }
}

impl TryFrom<HooqToml> for CheckedHooqToml {
    type Error = String;

    // ロード時に変換可能かをあらかじめ確認する
    fn try_from(value: HooqToml) -> Result<Self, Self::Error> {
        let _ = FlavorStore::try_from(value.clone())?;

        Ok(Self { inner: value })
    }
}
