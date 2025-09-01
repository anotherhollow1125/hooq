use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;

use proc_macro2::TokenStream;
use syn::{Expr, Path, Signature};

use crate::impls::inert_attr::InertAttrOption;
use crate::impls::root_attr::{RootOption, hook_method};
use crate::impls::utils::function_info::FunctionInfo;
use crate::impls::utils::path_is_end_of;

#[derive(Clone, Copy, Debug)]
pub enum HookTargetKind {
    Question,
    Return,
    TailExpr,
}

impl std::fmt::Display for HookTargetKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HookTargetKind::Question => write!(f, "?"),
            HookTargetKind::Return => write!(f, "return"),
            HookTargetKind::TailExpr => write!(f, "tail expr"),
        }
    }
}

#[derive(Debug)]
pub struct Counter {
    question: usize,
    return_: usize,
    tail_expr: usize,
}

impl Counter {
    pub fn new() -> Self {
        Self {
            question: 0,
            return_: 0,
            tail_expr: 0,
        }
    }

    pub fn count_up(&mut self, kind: HookTargetKind) {
        match kind {
            HookTargetKind::Question => self.question += 1,
            HookTargetKind::Return => self.return_ += 1,
            HookTargetKind::TailExpr => self.tail_expr += 1,
        }
    }

    pub fn get_count(&self, kind: HookTargetKind) -> usize {
        match kind {
            HookTargetKind::Question => self.question,
            HookTargetKind::Return => self.return_,
            HookTargetKind::TailExpr => self.tail_expr,
        }
    }
}

#[derive(Debug, Clone)]
pub enum LocalContextField<'a, T> {
    Inherit(&'a T),
    Override(T),
}

impl<'a, 'b: 'a, T> LocalContextField<'a, T> {
    fn from_parent(val: Option<T>, parent: &'b LocalContextField<'b, T>) -> Self {
        if let Some(val) = val {
            return Self::Override(val);
        }

        match parent {
            Self::Inherit(original) => Self::Inherit(original),
            Self::Override(val) => Self::Inherit(val),
        }
    }
}

impl<T> Deref for LocalContextField<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Inherit(local_context_kind) => local_context_kind,
            Self::Override(val) => val,
        }
    }
}

impl<'a, 'b: 'a, K, V> LocalContextField<'a, HashMap<K, Rc<V>>>
where
    K: std::hash::Hash + Eq + Clone,
{
    fn merged_from_parent(
        new_map: HashMap<K, Rc<V>>,
        parent: &'b LocalContextField<'b, HashMap<K, Rc<V>>>,
    ) -> Self {
        if new_map.is_empty() {
            return match parent {
                Self::Inherit(original) => Self::Inherit(original),
                Self::Override(val) => Self::Inherit(val),
            };
        }

        let merged_map = match parent {
            Self::Inherit(original) => {
                let mut map: HashMap<_, _> = original
                    .iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect();
                map.extend(new_map);
                map
            }
            Self::Override(val) => {
                let mut map: HashMap<_, _> =
                    val.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
                map.extend(new_map);
                map
            }
        };

        Self::Override(merged_map)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SkipStatus {
    SkipSameScope,
    SkipAll,
}

#[derive(Debug, Clone, Copy)]
pub struct HookTargetSwitch {
    pub question: bool,
    pub return_: bool,
    pub tail_expr: bool,
}

#[derive(Debug, Clone)]
pub struct LocalContext<'a> {
    // ユーザー設定値
    pub method: LocalContextField<'a, TokenStream>,
    pub hook_targets: LocalContextField<'a, HookTargetSwitch>,
    pub tail_expr_idents: LocalContextField<'a, Vec<String>>,
    pub result_types: LocalContextField<'a, Vec<String>>,
    pub hook_in_macros: LocalContextField<'a, bool>,
    pub bindings: LocalContextField<'a, HashMap<String, Rc<Expr>>>,
    pub skip_status: LocalContextField<'a, Option<SkipStatus>>,

    // 現在の関数情報
    pub fn_info: LocalContextField<'a, Option<FunctionInfo>>,
}

#[derive(Debug, Clone)]
pub struct HookContext<'a> {
    pub counter: Rc<RefCell<Counter>>,
    pub local_context: LocalContext<'a>,
}

impl<'a> HookContext<'a> {
    pub fn new<'b: 'a>(
        RootOption {
            trait_uses: _,
            method,
            hook_targets,
            tail_expr_idents,
            result_types,
            hook_in_macros,
            bindings,
            use_hook_method,
        }: RootOption,
    ) -> Self {
        let method = if use_hook_method {
            LocalContextField::Override(hook_method())
        } else {
            LocalContextField::Override(method)
        };

        Self {
            counter: Rc::new(RefCell::new(Counter::new())),
            local_context: LocalContext {
                method,
                hook_targets: LocalContextField::Override(hook_targets),
                tail_expr_idents: LocalContextField::Override(tail_expr_idents),
                result_types: LocalContextField::Override(result_types),
                hook_in_macros: LocalContextField::Override(hook_in_macros),
                bindings: LocalContextField::Override(bindings),
                skip_status: LocalContextField::Override(None),

                fn_info: LocalContextField::Override(None),
            },
        }
    }

    pub fn updated_by_inert_attr<'b: 'a>(
        parent_context: &'b HookContext<'b>,
        new_option: InertAttrOption,
    ) -> Self {
        let skip_status = LocalContextField::from_parent(
            new_option.get_skip_status().map(Option::Some),
            &parent_context.local_context.skip_status,
        );
        let InertAttrOption {
            method, bindings, ..
        } = new_option;

        let method = LocalContextField::from_parent(method, &parent_context.local_context.method);
        let hook_targets = LocalContextField::from_parent(
            None, // FIXME
            &parent_context.local_context.hook_targets,
        );
        let tail_expr_idents = LocalContextField::from_parent(
            None, // FIXME
            &parent_context.local_context.tail_expr_idents,
        );
        let result_types = LocalContextField::from_parent(
            None, // FIXME
            &parent_context.local_context.result_types,
        );
        let hook_in_macros = LocalContextField::from_parent(
            None, // FIXME
            &parent_context.local_context.hook_in_macros,
        );
        let bindings = LocalContextField::merged_from_parent(
            bindings.into_iter().map(|(k, v)| (k, Rc::new(v))).collect(),
            &parent_context.local_context.bindings,
        );

        // fn_info の更新は別タイミングで行う
        let fn_info = LocalContextField::from_parent(None, &parent_context.local_context.fn_info);

        Self {
            counter: Rc::clone(&parent_context.counter),
            local_context: LocalContext {
                method,
                hook_targets,
                tail_expr_idents,
                result_types,
                bindings,
                skip_status,
                hook_in_macros,
                fn_info,
            },
        }
    }

    pub fn as_hook_info(&'a self, expr: &'a str, kind: HookTargetKind) -> HookInfo<'a> {
        HookInfo {
            expr,
            kind,

            hook_context: self,
        }
    }

    pub fn for_sub_scope_context(&self) -> Self {
        Self {
            counter: self.counter.clone(),
            local_context: LocalContext {
                skip_status: match self.local_context.skip_status {
                    LocalContextField::Inherit(&Some(SkipStatus::SkipAll))
                    | LocalContextField::Override(Some(SkipStatus::SkipAll)) => {
                        LocalContextField::Override(Some(SkipStatus::SkipAll))
                    }
                    _ => LocalContextField::Override(None),
                },
                ..self.local_context.clone()
            },
        }
    }

    pub fn update_fn_info(&mut self, sig: &Signature) {
        self.local_context.fn_info =
            LocalContextField::Override(Some(FunctionInfo::new(sig.clone())));
    }

    pub fn is_skiped(&self) -> bool {
        self.local_context.skip_status.as_ref().is_some()
    }

    pub fn hook_in_macros(&self) -> bool {
        *self.local_context.hook_in_macros
    }

    pub fn path_is_special_call_like_err(&self, path: &Path) -> bool {
        self.local_context
            .tail_expr_idents
            .iter()
            .any(|s| path_is_end_of(path, s))
    }

    pub fn return_type_is_result(&self) -> bool {
        let result_types = self.local_context.result_types.as_slice();

        self.local_context
            .fn_info
            .as_ref()
            .map(|info| info.return_type_is_result(result_types))
            .unwrap_or(false)
    }
}

#[derive(Debug)]
pub struct HookInfo<'a> {
    pub expr: &'a str,
    pub kind: HookTargetKind,

    pub hook_context: &'a HookContext<'a>,
}

impl HookInfo<'_> {
    pub fn counter(&self) -> Rc<RefCell<Counter>> {
        Rc::clone(&self.hook_context.counter)
    }

    pub fn fn_info(&self) -> Option<&FunctionInfo> {
        self.hook_context.local_context.fn_info.as_ref()
    }

    pub fn method(&self) -> &TokenStream {
        &self.hook_context.local_context.method
    }

    pub fn is_hook_target(&self) -> bool {
        match self.kind {
            HookTargetKind::Question => self.hook_context.local_context.hook_targets.question,
            HookTargetKind::Return => self.hook_context.local_context.hook_targets.return_,
            HookTargetKind::TailExpr => self.hook_context.local_context.hook_targets.tail_expr,
        }
    }

    pub fn available_bindings(&self) -> Vec<(String, Rc<Expr>)> {
        let mut v: Vec<_> = self
            .hook_context
            .local_context
            .bindings
            .iter()
            .map(|(k, v)| (k.clone(), Rc::clone(v)))
            .collect();

        // NOTE:
        // スナップショットテスト通過のために出力が一意になるようソート
        // パフォーマンス的に行いたくないという気持ちもあるが、
        // そもそもマクロの実行結果が実行ごとに異なるのも良くないため、行ってよい処理と判断
        v.sort_by(|(k1, _), (k2, _)| k1.cmp(k2));

        v
    }

    pub fn get_binding(&self, key: &str) -> Option<&Expr> {
        self.hook_context
            .local_context
            .bindings
            .get(key)
            .map(|rc| rc.as_ref())
    }
}
