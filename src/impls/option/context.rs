use std::{cell::RefCell, rc::Rc};

use proc_macro2::TokenStream;
use quote::ToTokens;

use crate::impls::utils::return_type_is_result;

#[derive(Clone, Debug)]
pub struct FunctionInfo {
    pub name: String,
    pub sig: String,
    pub return_type_is_result: bool,
}

pub trait ExtractFunctionInfo {
    fn extract_function_info(&self) -> syn::Result<FunctionInfo>;
}

impl ExtractFunctionInfo for syn::ItemFn {
    fn extract_function_info(&self) -> syn::Result<FunctionInfo> {
        let sig = self.sig.to_token_stream().to_string();
        let name = self.sig.ident.to_string();
        let return_type_is_result = return_type_is_result(&self.sig.output);

        Ok(FunctionInfo {
            name,
            sig,
            return_type_is_result,
        })
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ReplaceKind {
    Question,
    Return,
    TailExpr,
}

impl std::fmt::Display for ReplaceKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReplaceKind::Question => write!(f, "?"),
            ReplaceKind::Return => write!(f, "return"),
            ReplaceKind::TailExpr => write!(f, "tail expr"),
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

    pub fn count_up(&mut self, kind: ReplaceKind) {
        match kind {
            ReplaceKind::Question => self.question += 1,
            ReplaceKind::Return => self.return_ += 1,
            ReplaceKind::TailExpr => self.tail_expr += 1,
        }
    }

    pub fn get_count(&self, kind: ReplaceKind) -> usize {
        match kind {
            ReplaceKind::Question => self.question,
            ReplaceKind::Return => self.return_,
            ReplaceKind::TailExpr => self.tail_expr,
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
    pub override_method: LocalContextField<'a, TokenStream>,
    pub return_type_is_result: LocalContextField<'a, bool>,
}

#[derive(Debug, Clone)]
pub struct PartialReplaceContext<'a> {
    pub counter: Rc<RefCell<Counter>>,
    pub fn_info: &'a FunctionInfo,
    pub local_context: LocalContext<'a>,
}

impl<'a> PartialReplaceContext<'a> {
    pub fn new_root<'b: 'a>(
        fn_info: &'b FunctionInfo,
        skip_status: Option<SkipStatus>,
        tag: Option<String>,
        override_method: Option<TokenStream>,
    ) -> Self {
        Self {
            counter: Rc::new(RefCell::new(Counter::new())),
            fn_info,
            local_context: LocalContext {
                skip_status: LocalContextField::new_from_option(skip_status),
                tag: LocalContextField::new_from_option(tag),
                override_method: LocalContextField::new_from_option(override_method),
                return_type_is_result: LocalContextField::Override(fn_info.return_type_is_result),
            },
        }
    }

    pub fn new<'b: 'a>(
        parent_context: &'b PartialReplaceContext<'b>,
        tag: Option<String>,
        override_method: Option<TokenStream>,
        return_type_is_result: Option<bool>,
        skip_status: Option<SkipStatus>,
    ) -> Self {
        let tag = LocalContextField::from_parent(tag, &parent_context.local_context.tag);
        let override_method = LocalContextField::from_parent(
            override_method,
            &parent_context.local_context.override_method,
        );
        let return_type_is_result = LocalContextField::from_parent(
            return_type_is_result,
            &parent_context.local_context.return_type_is_result,
        );
        let skip_status =
            LocalContextField::from_parent(skip_status, &parent_context.local_context.skip_status);

        Self {
            counter: Rc::clone(&parent_context.counter),
            fn_info: parent_context.fn_info,
            local_context: LocalContext {
                skip_status,
                tag,
                override_method,
                return_type_is_result,
            },
        }
    }

    pub fn as_replace_context(&'a self, expr: String, kind: ReplaceKind) -> ReplaceContext<'a> {
        ReplaceContext {
            expr,
            kind,

            partial_replace_context: self,
        }
    }

    pub fn for_sub_scope_context(&self) -> Self {
        Self {
            counter: self.counter.clone(),
            fn_info: self.fn_info,
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
    pub fn update_override_method(&mut self, override_method: TokenStream) {
        self.local_context.override_method = LocalContextField::Override(override_method);
    }
    */

    pub fn update_return_type_is_result(&mut self, is_result: bool) {
        self.local_context.return_type_is_result = LocalContextField::Override(is_result);
    }

    /*
    pub fn tag(&self) -> Option<&String> {
        self.local_context.tag.as_ref()
    }
    */

    /*
    pub fn override_method(&self) -> Option<&TokenStream> {
        self.local_context.override_method.as_ref()
    }
    */

    pub fn is_skiped(&self) -> bool {
        self.local_context.skip_status.is_some()
    }

    pub fn return_type_is_result(&self) -> bool {
        *self
            .local_context
            .return_type_is_result
            .as_ref()
            .unwrap_or(&false)
    }
}

#[derive(Debug)]
pub struct ReplaceContext<'a> {
    pub expr: String,
    pub kind: ReplaceKind,

    pub partial_replace_context: &'a PartialReplaceContext<'a>,
}

impl ReplaceContext<'_> {
    pub fn counter(&self) -> Rc<RefCell<Counter>> {
        Rc::clone(&self.partial_replace_context.counter)
    }

    pub fn fn_info(&self) -> &FunctionInfo {
        self.partial_replace_context.fn_info
    }

    pub fn tag(&self) -> Option<&String> {
        self.partial_replace_context.local_context.tag.as_ref()
    }

    pub fn override_method(&self) -> Option<&TokenStream> {
        self.partial_replace_context
            .local_context
            .override_method
            .as_ref()
    }

    /*
    pub fn return_type_is_result(&self) -> bool {
        *self
            .partial_replace_context
            .local_context
            .return_type_is_result
            .as_ref()
            .unwrap_or(&false)
    }
    */
}
