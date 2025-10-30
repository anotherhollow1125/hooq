use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::{LazyLock, Mutex};

use proc_macro2::TokenStream;
use syn::{Expr, Path, parse_quote};

pub use crate::impls::flavor::flavor_path::FlavorPath;
use crate::impls::flavor::toml_load::HooqToml;
use crate::impls::inert_attr::context::HookTargetSwitch;
use crate::impls::method::Method;

mod flavor_path;
mod presets;
mod toml_load;

#[derive(Debug, Clone)]
pub struct Flavor {
    pub trait_uses: Vec<Path>,
    pub method: Method,
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
            method: default_method().into(),
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
            let col = $col;
            let expr = $expr_str_short;

            ::std::eprintln!("[{path}:{line}:{col}] {e:?}
    {expr}");
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

pub static LOADED_HOOQ_TOML: LazyLock<TomlStore> = LazyLock::new(|| TomlStore {
    inner: Mutex::new(HashMap::new()),
});

impl TomlStore {
    fn load() -> Result<Option<CheckedHooqToml>, String> {
        if let Some(checked_hooq_toml) = LOADED_HOOQ_TOML.get() {
            return Ok(Some(checked_hooq_toml));
        }

        let dir_path = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
        let path = PathBuf::from(dir_path).join("hooq.toml");

        if let Ok(false) | Err(_) = path.try_exists() {
            return Ok(None);
        }

        let content = std::fs::read_to_string(&path)
            .map_err(|e| format!("failed to read file `{}`: {}", path.display(), e))?;

        let hooq_toml: HooqToml = toml::from_str(&content)
            .map_err(|e| format!("failed to parse toml from file `{}`: {}", path.display(), e))?;

        let checked_hooq_toml = CheckedHooqToml::try_from(hooq_toml)?;

        LOADED_HOOQ_TOML.set(checked_hooq_toml.clone());

        Ok(Some(checked_hooq_toml))
    }

    // NOTE:
    // 異なるCargoプロジェクトを同じタイミングでVSCodeで開いていた時に
    // 違うTomlStoreの内容が違うプロジェクトに供給されている事象が見られた
    //
    // 効果がどれぐらいあるかは懐疑的であるが、少しでも軽減すべく
    // プロジェクトごとに保存領域を分けるようにした
    fn set(&self, checked_hooq_toml: CheckedHooqToml) {
        let key = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());

        self.inner.lock().unwrap().insert(key, checked_hooq_toml);
    }

    fn get(&self) -> Option<CheckedHooqToml> {
        let key = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());

        self.inner.lock().unwrap().get(&key).cloned()
    }
}

impl FlavorStore {
    fn new() -> Self {
        let flavors = presets::preset_flavors();

        Self { flavors }
    }

    pub fn with_hooq_toml() -> Result<Self, String> {
        let mut flavors = Self::new();

        if let Some(hooq_toml) = TomlStore::load()? {
            // 変換が成功することはCheckedHooqTomlの生成時に確認済み
            toml_load::apply::update_flavors(&mut flavors.flavors, hooq_toml.inner).unwrap();
        }

        Ok(flavors)
    }

    fn get_flavor_inner(&self, path: &FlavorPath) -> Option<Flavor> {
        let mut path = path.iter();
        let mut current: &Flavor = self.flavors.get(path.next()?)?;

        for name in path {
            current = current.sub_flavors.get(name)?;
        }

        Some(current.clone())
    }

    pub fn get_flavor(&self, path: &FlavorPath) -> Result<Flavor, String> {
        self.get_flavor_inner(path).ok_or_else(|| {
            format!(
                "flavor `{}` is not found. available flavors:
{}",
                path.join("::"),
                self.all_flavor_names()
                    .into_iter()
                    .map(|name| format!("  - {name}"))
                    .collect::<Vec<_>>()
                    .join("\n")
            )
        })
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
