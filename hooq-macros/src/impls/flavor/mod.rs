use std::collections::HashMap;
use std::rc::Rc;
use std::sync::OnceLock;

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

pub struct FlavorStore {
    flavors: HashMap<String, Flavor>,
}

// NOTE:
// 本当はパース後のFlavorsをグローバルに保持したいが、OnceLockはSync制約があるため
// TokenStreamを使っているFlavorsはそのままでは保持できない
// そのため変換確認だけ行った状態でHooqTomlをグローバルに保持し、
// 実際にFlavorsが必要な時にはunwrapを許可することにした
#[derive(Debug)]
pub struct CheckedHooqToml {
    pub inner: HooqToml,
}

pub static LOADED_HOOQ_TOML: OnceLock<CheckedHooqToml> = OnceLock::new();

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
            toml_load::parse::update_flavors(&mut flavors.flavors, hooq_toml.inner.clone())
                .unwrap();
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
}

impl TryFrom<HooqToml> for FlavorStore {
    type Error = syn::Error;

    fn try_from(value: HooqToml) -> Result<Self, Self::Error> {
        let mut flavors = Self::new();

        toml_load::parse::update_flavors(&mut flavors.flavors, value)?;

        Ok(flavors)
    }
}

impl TryFrom<HooqToml> for CheckedHooqToml {
    type Error = syn::Error;

    // ロード時に変換可能かをあらかじめ確認する
    fn try_from(value: HooqToml) -> Result<Self, Self::Error> {
        let _ = FlavorStore::try_from(value.clone())?;

        Ok(Self { inner: value })
    }
}
