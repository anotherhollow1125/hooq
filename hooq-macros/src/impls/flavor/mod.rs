use std::cell::RefCell;
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
use crate::impls::utils::unexpected_error_message::UNEXPECTED_ERROR_MESSAGE;

mod flavor_path;
mod presets;
mod toml_load;

#[derive(Debug, Clone)]
pub enum FlavorSettingField<T> {
    Base(T),
    Inherit(Rc<RefCell<FlavorSettingField<T>>>),
}

impl<T> FlavorSettingField<T> {
    pub fn new(value: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(FlavorSettingField::Base(value)))
    }

    pub fn set(this: &Rc<RefCell<Self>>, value: T) {
        *this.borrow_mut() = FlavorSettingField::Base(value);
    }
}

impl<T: Clone> FlavorSettingField<T> {
    pub fn clone_inner(&self) -> T {
        match self {
            FlavorSettingField::Base(value) => value.clone(),
            FlavorSettingField::Inherit(parent) => parent.borrow().clone_inner(),
        }
    }
}

#[derive(Debug)]
pub struct FlavorNode {
    pub settings: FlavorSettings,
    pub sub_flavors: HashMap<String, FlavorNode>,
}

#[derive(Debug)]
pub struct FlavorSettings {
    pub trait_uses: Rc<RefCell<FlavorSettingField<Vec<Path>>>>,
    pub method: Rc<RefCell<FlavorSettingField<Method>>>,
    pub hook_targets: Rc<RefCell<FlavorSettingField<HookTargetSwitch>>>,
    pub tail_expr_idents: Rc<RefCell<FlavorSettingField<Vec<String>>>>,
    pub ignore_tail_expr_idents: Rc<RefCell<FlavorSettingField<Vec<String>>>>,
    pub result_types: Rc<RefCell<FlavorSettingField<Vec<String>>>>,
    pub hook_in_macros: Rc<RefCell<FlavorSettingField<bool>>>,
    #[allow(clippy::type_complexity)]
    pub bindings: Rc<RefCell<FlavorSettingField<HashMap<String, Rc<Expr>>>>>,
}

#[derive(Debug, Clone)]
pub struct FlavorInstance {
    pub trait_uses: Vec<Path>,
    pub method: Method,
    pub hook_targets: HookTargetSwitch,
    pub tail_expr_idents: Vec<String>,
    pub ignore_tail_expr_idents: Vec<String>,
    pub result_types: Vec<String>,
    pub hook_in_macros: bool,
    pub bindings: HashMap<String, Rc<Expr>>,
}

impl From<&FlavorSettings> for FlavorInstance {
    fn from(value: &FlavorSettings) -> Self {
        Self {
            trait_uses: value.trait_uses.borrow().clone_inner(),
            method: value.method.borrow().clone_inner(),
            hook_targets: value.hook_targets.borrow().clone_inner(),
            tail_expr_idents: value.tail_expr_idents.borrow().clone_inner(),
            ignore_tail_expr_idents: value.ignore_tail_expr_idents.borrow().clone_inner(),
            result_types: value.result_types.borrow().clone_inner(),
            hook_in_macros: value.hook_in_macros.borrow().clone_inner(),
            bindings: value.bindings.borrow().clone_inner(),
        }
    }
}

impl Default for FlavorSettings {
    fn default() -> Self {
        Self {
            trait_uses: FlavorSettingField::new(Vec::new()),
            method: FlavorSettingField::new(default_method()),
            hook_targets: FlavorSettingField::new(HookTargetSwitch {
                question: true,
                return_: true,
                tail_expr: true,
            }),
            tail_expr_idents: FlavorSettingField::new(vec!["Err".to_string()]),
            ignore_tail_expr_idents: FlavorSettingField::new(vec!["Ok".to_string()]),
            result_types: FlavorSettingField::new(vec!["Result".to_string()]),
            hook_in_macros: FlavorSettingField::new(true),
            bindings: FlavorSettingField::new(HashMap::new()),
        }
    }
}

fn default_method() -> Method {
    // NOTE:
    // $path や $line は eprintln! に直接埋め込みたいところだが、
    // CI側のテストの関係でこのようになっている

    let excerpted_helpers_path = crate::impls::utils::get_source_excerpt_helpers_name_space();

    let res: TokenStream = parse_quote! {
        .inspect_err(|e| {
            let path = $path;
            let line = $line;
            let col = $col;
            let expr = #excerpted_helpers_path ::excerpted_pretty_stringify!($source);

            ::std::eprintln!("[{path}:{line}:{col}] {e:?}\n{expr}");
        })
    };

    Method::try_from(res).expect(UNEXPECTED_ERROR_MESSAGE)
}

impl FlavorSettings {
    pub fn prepare_inheritance(&self) -> FlavorSettings {
        FlavorSettings {
            trait_uses: Rc::new(RefCell::new(FlavorSettingField::Inherit(Rc::clone(
                &self.trait_uses,
            )))),
            method: Rc::new(RefCell::new(FlavorSettingField::Inherit(Rc::clone(
                &self.method,
            )))),
            hook_targets: Rc::new(RefCell::new(FlavorSettingField::Inherit(Rc::clone(
                &self.hook_targets,
            )))),
            tail_expr_idents: Rc::new(RefCell::new(FlavorSettingField::Inherit(Rc::clone(
                &self.tail_expr_idents,
            )))),
            ignore_tail_expr_idents: Rc::new(RefCell::new(FlavorSettingField::Inherit(Rc::clone(
                &self.ignore_tail_expr_idents,
            )))),
            result_types: Rc::new(RefCell::new(FlavorSettingField::Inherit(Rc::clone(
                &self.result_types,
            )))),
            hook_in_macros: Rc::new(RefCell::new(FlavorSettingField::Inherit(Rc::clone(
                &self.hook_in_macros,
            )))),
            bindings: Rc::new(RefCell::new(FlavorSettingField::Inherit(Rc::clone(
                &self.bindings,
            )))),
        }
    }
}

#[derive(Debug)]
pub struct FlavorStore {
    flavors: HashMap<String, FlavorNode>,
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

    fn get_flavor_inner(&self, path: &FlavorPath) -> Option<&FlavorNode> {
        let mut path = path.iter();
        let mut current: &FlavorNode = self.flavors.get(path.next()?)?;

        for name in path {
            current = current.sub_flavors.get(name)?;
        }

        Some(current)
    }

    pub fn get_flavor(&self, path: &FlavorPath) -> Result<FlavorInstance, String> {
        self.get_flavor_inner(path)
            .map(|node| (&node.settings).into())
            .ok_or_else(|| {
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
        fn collect_names(flavor: &FlavorNode, prefix: &str, names: &mut Vec<String>) {
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
