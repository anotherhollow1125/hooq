use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;

use proc_macro2::TokenStream;
use syn::{Expr, Path};

use crate::impls::inert_attr::InertAttribute;
use crate::impls::method::Method;
use crate::impls::root_attr::RootContext;
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
pub struct LocalContextField<T> {
    val: Rc<T>,
    // overridden_ancestor は異なる設定値を持つ祖先を表す
    // 子やXxxContextは親を参照するが、親が子を参照することはないため、
    // WeakではなくRcで保持しても循環参照にはならない
    overridden_ancestor: Option<Rc<Self>>,
}

impl<T> LocalContextField<T> {
    fn new(val: T) -> Rc<Self> {
        Rc::new(Self {
            val: Rc::new(val),
            overridden_ancestor: None,
        })
    }

    fn from_parent(val: Option<T>, parent: &Rc<Self>) -> Rc<Self> {
        match val {
            Some(val) => Rc::new(Self {
                // 親から上書き
                val: Rc::new(val),
                overridden_ancestor: Some(Rc::clone(parent)),
            }),
            None => Rc::clone(parent), // 親をそのまま引き継ぐ
        }
    }

    pub fn get_overridden_ancestor(&self) -> Option<Rc<Self>> {
        self.overridden_ancestor.clone()
    }
}

impl<T> Deref for LocalContextField<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.val
    }
}

impl<K, V> LocalContextField<HashMap<K, Rc<V>>>
where
    K: std::hash::Hash + Eq + Clone,
{
    fn merged_from_parent(
        new_map: HashMap<K, Rc<V>>,
        parent: &Rc<LocalContextField<HashMap<K, Rc<V>>>>,
    ) -> Rc<Self> {
        if new_map.is_empty() {
            return Self::from_parent(None, parent);
        }

        let mut map: HashMap<_, _> = parent
            .val
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        map.extend(new_map);

        Self::from_parent(Some(map), parent)
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

impl TryFrom<Vec<String>> for HookTargetSwitch {
    type Error = String;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        let mut switch = HookTargetSwitch {
            question: false,
            return_: false,
            tail_expr: false,
        };

        for s in value {
            match s.as_str() {
                "?" | "question" => switch.question = true,
                "return" => switch.return_ = true,
                "tail_expr" | "tail expr" | "tailexpr" | "tailExpr" => switch.tail_expr = true,
                "all" => {
                    switch.question = true;
                    switch.return_ = true;
                    switch.tail_expr = true;
                }
                e => return Err(e.to_string()),
            }
        }

        Ok(switch)
    }
}

#[derive(Debug, Clone)]
pub struct LocalContext {
    // ユーザー設定値
    pub method: Rc<LocalContextField<Method>>,
    pub hook_targets: Rc<LocalContextField<HookTargetSwitch>>,
    pub tail_expr_idents: Rc<LocalContextField<Vec<String>>>,
    pub ignore_tail_expr_idents: Rc<LocalContextField<Vec<String>>>,
    pub result_types: Rc<LocalContextField<Vec<String>>>,
    pub hook_in_macros: Rc<LocalContextField<bool>>,
    pub bindings: Rc<LocalContextField<HashMap<String, Rc<Expr>>>>,
    pub skip_status: Rc<LocalContextField<Option<SkipStatus>>>,

    // 現在の関数情報
    pub fn_info: Rc<LocalContextField<Option<FunctionInfo>>>,
}

#[derive(Debug, Clone)]
pub struct HookContext {
    pub counter: Rc<RefCell<Counter>>,
    pub local_context: LocalContext,
}

impl HookContext {
    pub fn new(
        RootContext {
            trait_uses: _,
            method,
            hook_targets,
            tail_expr_idents,
            ignore_tail_expr_idents,
            result_types,
            hook_in_macros,
            bindings,
        }: RootContext,
    ) -> Self {
        let method = LocalContextField::new(method);

        Self {
            counter: Rc::new(RefCell::new(Counter::new())),
            local_context: LocalContext {
                method,
                hook_targets: LocalContextField::new(hook_targets),
                tail_expr_idents: LocalContextField::new(tail_expr_idents),
                ignore_tail_expr_idents: LocalContextField::new(ignore_tail_expr_idents),
                result_types: LocalContextField::new(result_types),
                hook_in_macros: LocalContextField::new(hook_in_macros),
                bindings: LocalContextField::new(bindings),
                skip_status: LocalContextField::new(None),

                fn_info: LocalContextField::new(None),
            },
        }
    }

    pub fn updated_by_inert_attr(parent_context: &HookContext, inert_attr: InertAttribute) -> Self {
        let skip_status = LocalContextField::from_parent(
            inert_attr.get_skip_status().map(Option::Some),
            &parent_context.local_context.skip_status,
        );
        let InertAttribute {
            method,
            hook_targets,
            tail_expr_idents,
            ignore_tail_expr_idents,
            result_types,
            hook_in_macros,
            bindings,
            ..
        } = inert_attr;

        let method = LocalContextField::from_parent(method, &parent_context.local_context.method);
        let hook_targets = LocalContextField::from_parent(
            hook_targets,
            &parent_context.local_context.hook_targets,
        );
        let tail_expr_idents = LocalContextField::from_parent(
            tail_expr_idents,
            &parent_context.local_context.tail_expr_idents,
        );
        let ignore_tail_expr_idents = LocalContextField::from_parent(
            ignore_tail_expr_idents,
            &parent_context.local_context.ignore_tail_expr_idents,
        );
        let result_types = LocalContextField::from_parent(
            result_types,
            &parent_context.local_context.result_types,
        );
        let hook_in_macros = LocalContextField::from_parent(
            hook_in_macros,
            &parent_context.local_context.hook_in_macros,
        );
        let bindings = LocalContextField::merged_from_parent(
            bindings.into_iter().map(|(k, v)| (k, Rc::new(v))).collect(),
            &parent_context.local_context.bindings,
        );

        // fn_info の更新は別タイミングで行う
        let fn_info = Rc::clone(&parent_context.local_context.fn_info);

        Self {
            counter: Rc::clone(&parent_context.counter),
            local_context: LocalContext {
                method,
                hook_targets,
                tail_expr_idents,
                ignore_tail_expr_idents,
                result_types,
                bindings,
                skip_status,
                hook_in_macros,
                fn_info,
            },
        }
    }

    pub fn as_hook_info<'a>(
        &'a self,
        source_tokenstream: TokenStream,
        kind: HookTargetKind,
    ) -> HookInfo<'a> {
        HookInfo {
            source_tokenstream,
            kind,

            hook_context: self,
        }
    }

    pub fn for_sub_scope_context(&self) -> Self {
        let skip_status = match &**self.local_context.skip_status {
            Some(SkipStatus::SkipAll) => LocalContextField::from_parent(
                Some(Some(SkipStatus::SkipAll)),
                &self.local_context.skip_status,
            ),
            _ => LocalContextField::from_parent(Some(None), &self.local_context.skip_status),
        };

        Self {
            counter: self.counter.clone(),
            local_context: LocalContext {
                skip_status,
                ..self.local_context.clone()
            },
        }
    }

    pub fn update_fn_info(&mut self, function_info: FunctionInfo) {
        self.local_context.fn_info =
            LocalContextField::from_parent(Some(Some(function_info)), &self.local_context.fn_info);
    }

    pub fn is_skipped(&self) -> bool {
        self.local_context.skip_status.as_ref().is_some()
    }

    pub fn hook_in_macros(&self) -> bool {
        **self.local_context.hook_in_macros
    }

    pub fn path_is_hook_call_like_err(&self, path: &Path) -> bool {
        self.local_context
            .tail_expr_idents
            .iter()
            .any(|s| path_is_end_of(path, s))
    }

    pub fn path_is_not_hook_call_like_ok(&self, path: &Path) -> bool {
        self.local_context
            .ignore_tail_expr_idents
            .iter()
            .any(|s| path_is_end_of(path, s))
    }

    pub fn return_type_is_result(&self) -> bool {
        let result_types = self.local_context.result_types.as_slice();

        self.local_context
            .fn_info
            .as_ref()
            .as_ref()
            .map(|info| info.return_type_is_result(result_types))
            .unwrap_or(false)
    }

    pub fn get_overridden_ancestor_of_method(&self) -> Option<Rc<LocalContextField<Method>>> {
        self.local_context.method.get_overridden_ancestor()
    }
}

#[derive(Debug)]
pub struct HookInfo<'a> {
    pub source_tokenstream: TokenStream,
    pub kind: HookTargetKind,

    pub hook_context: &'a HookContext,
}

impl HookInfo<'_> {
    pub fn counter(&self) -> Rc<RefCell<Counter>> {
        Rc::clone(&self.hook_context.counter)
    }

    pub fn fn_info(&self) -> Option<&FunctionInfo> {
        self.hook_context.local_context.fn_info.as_ref().as_ref()
    }

    pub fn method(&self) -> &Method {
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
