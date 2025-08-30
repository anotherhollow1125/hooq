use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use proc_macro2::TokenStream;
use syn::{Expr, Signature};

use crate::impls::inert_attr::InertAttrOption;
use crate::impls::inert_attr::method::method_for_custom;
use crate::impls::root_attr::HooqRootOption;
use crate::impls::utils::function_info::FunctionInfo;

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
    None,
    Inherit(&'a T),
    Override(T),
}

impl<'a, 'b: 'a, T> LocalContextField<'a, T> {
    fn from_parent(val: Option<T>, parent: &'b LocalContextField<'b, T>) -> Self {
        if let Some(val) = val {
            return Self::Override(val);
        }

        match parent {
            Self::None => Self::None,
            Self::Inherit(original) => Self::Inherit(original),
            Self::Override(val) => Self::Inherit(val),
        }
    }

    fn as_ref(&self) -> Option<&T> {
        match self {
            Self::None => None,
            Self::Inherit(local_context_kind) => Some(local_context_kind),
            Self::Override(val) => Some(val),
        }
    }

    fn is_some(&self) -> bool {
        matches!(self, Self::Inherit(_) | Self::Override(_))
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
                Self::None => Self::None,
                Self::Inherit(original) => Self::Inherit(original),
                Self::Override(val) => Self::Inherit(val),
            };
        }

        let merged_map = match parent {
            Self::None => new_map,
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

#[derive(Debug, Clone)]
pub struct LocalContext<'a> {
    pub skip_status: LocalContextField<'a, SkipStatus>,
    pub method: LocalContextField<'a, TokenStream>,
    pub fn_info: LocalContextField<'a, FunctionInfo>,
    pub bindings: LocalContextField<'a, HashMap<String, Rc<Expr>>>,
}

#[derive(Debug, Clone)]
pub struct HookContext<'a> {
    pub counter: Rc<RefCell<Counter>>,
    pub local_context: LocalContext<'a>,
}

impl<'a> HookContext<'a> {
    pub fn new<'b: 'a>(root_option: &HooqRootOption) -> Self {
        let method = if root_option.is_custom {
            LocalContextField::Override(method_for_custom())
        } else {
            LocalContextField::None
        };

        Self {
            counter: Rc::new(RefCell::new(Counter::new())),
            local_context: LocalContext {
                skip_status: LocalContextField::None,
                method,
                fn_info: LocalContextField::None,
                bindings: LocalContextField::None,
            },
        }
    }

    pub fn updated_by_inert_attr<'b: 'a>(
        parent_context: &'b HookContext<'b>,
        new_option: InertAttrOption,
    ) -> Self {
        let skip_status = LocalContextField::from_parent(
            new_option.get_skip_status(),
            &parent_context.local_context.skip_status,
        );
        let InertAttrOption {
            method, bindings, ..
        } = new_option;

        let method = LocalContextField::from_parent(method, &parent_context.local_context.method);
        // fn_info の更新は別タイミングで行う
        let fn_info = LocalContextField::from_parent(None, &parent_context.local_context.fn_info);
        let bindings = LocalContextField::merged_from_parent(
            bindings.into_iter().map(|(k, v)| (k, Rc::new(v))).collect(),
            &parent_context.local_context.bindings,
        );

        Self {
            counter: Rc::clone(&parent_context.counter),
            local_context: LocalContext {
                skip_status,
                method,
                fn_info,
                bindings,
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
                    LocalContextField::Inherit(&SkipStatus::SkipAll)
                    | LocalContextField::Override(SkipStatus::SkipAll) => {
                        LocalContextField::Override(SkipStatus::SkipAll)
                    }
                    _ => LocalContextField::None,
                },
                ..self.local_context.clone()
            },
        }
    }

    pub fn update_fn_info(&mut self, sig: &Signature) {
        self.local_context.fn_info = LocalContextField::Override(FunctionInfo::new(sig.clone()));
    }

    pub fn is_skiped(&self) -> bool {
        self.local_context.skip_status.is_some()
    }

    pub fn return_type_is_result(&self) -> bool {
        self.local_context
            .fn_info
            .as_ref()
            .map(|info| info.return_type_is_result())
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

    pub fn method(&self) -> Option<&TokenStream> {
        self.hook_context.local_context.method.as_ref()
    }

    pub fn get_binding(&self, key: &str) -> Option<&Expr> {
        self.hook_context
            .local_context
            .bindings
            .as_ref()
            .and_then(|map| map.get(key).map(|rc| rc.as_ref()))
    }
}
