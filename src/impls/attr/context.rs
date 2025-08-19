use std::cell::RefCell;
use std::rc::Rc;

use proc_macro2::TokenStream;
use syn::Signature;

use crate::impls::attr::inert_attr::InertAttrOption;
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
    fn new_from_option(val: Option<T>) -> Self {
        match val {
            Some(v) => LocalContextField::Override(v),
            None => LocalContextField::None,
        }
    }

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

#[derive(Debug, Clone, Copy)]
pub enum SkipStatus {
    SkipSameScope,
    SkipAll,
}

#[derive(Debug, Clone)]
pub struct LocalContext<'a> {
    pub skip_status: LocalContextField<'a, SkipStatus>,
    pub tag: LocalContextField<'a, String>,
    pub method: LocalContextField<'a, TokenStream>,
    pub fn_info: LocalContextField<'a, FunctionInfo>,
}

#[derive(Debug, Clone)]
pub struct HookContext<'a> {
    pub counter: Rc<RefCell<Counter>>,
    pub local_context: LocalContext<'a>,
}

impl<'a> HookContext<'a> {
    pub fn init<'b: 'a>(inert_attr_option: InertAttrOption) -> Self {
        Self {
            counter: Rc::new(RefCell::new(Counter::new())),
            local_context: LocalContext {
                skip_status: LocalContextField::new_from_option(
                    inert_attr_option.get_skip_status(),
                ),
                tag: LocalContextField::new_from_option(inert_attr_option.tag),
                method: LocalContextField::new_from_option(inert_attr_option.method),
                fn_info: LocalContextField::None,
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
        let InertAttrOption { tag, method, .. } = new_option;

        let tag = LocalContextField::from_parent(tag, &parent_context.local_context.tag);
        let method = LocalContextField::from_parent(method, &parent_context.local_context.method);
        // fn_info の更新は別タイミングで行う
        let fn_info = LocalContextField::from_parent(None, &parent_context.local_context.fn_info);

        Self {
            counter: Rc::clone(&parent_context.counter),
            local_context: LocalContext {
                skip_status,
                tag,
                method,
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

    /*
    pub fn update_is_skiped_all(&mut self, is_skiped_all: bool) {
        self.local_context.is_skiped_all =
            LocalContextField::Override(is_skiped_all);
    }
    */

    /*
    pub fn update_tag(&mut self, tag: String) {
        self.local_context.tag = LocalContextField::Override(tag);
    }
    */

    /*
    pub fn update_method(&mut self, method: TokenStream) {
        self.local_context.method = LocalContextField::Override(method);
    }
    */

    pub fn update_fn_info(&mut self, sig: &Signature) {
        self.local_context.fn_info = LocalContextField::Override(FunctionInfo::new(sig.clone()));
    }

    /*
    pub fn tag(&self) -> Option<&String> {
        self.local_context.tag.as_ref()
    }
    */

    /*
    pub fn method(&self) -> Option<&TokenStream> {
        self.local_context.method.as_ref()
    }
    */

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

    pub fn tag(&self) -> Option<&String> {
        self.hook_context.local_context.tag.as_ref()
    }

    pub fn method(&self) -> Option<&TokenStream> {
        self.hook_context.local_context.method.as_ref()
    }
}
