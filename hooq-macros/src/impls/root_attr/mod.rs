use std::collections::HashMap;
use std::rc::Rc;

use proc_macro2::TokenStream;
use syn::{Expr, Path, parse_quote};

use crate::impls::inert_attr::context::HookTargetSwitch;

mod parse;

#[derive(Debug)]
pub struct RootContext {
    // root_attr でないと設定不可能なフィールド
    pub trait_uses: Vec<Path>,

    // inert_attr でも設定可能
    // flavor の内容を反映させる
    pub method: TokenStream,
    pub hook_targets: HookTargetSwitch,
    pub tail_expr_idents: Vec<String>,
    pub result_types: Vec<String>,
    pub hook_in_macros: bool,
    pub bindings: HashMap<String, Rc<Expr>>,
}

impl RootContext {
    pub fn trait_uses_token_stream(&self) -> TokenStream {
        self.trait_uses
            .iter()
            .map(|p| quote::quote! { # [allow(unused)] use #p as _; })
            .collect()
    }
}

#[derive(Debug)]
pub struct RootAttribute {
    pub trait_uses: Vec<Path>,
    #[allow(unused)] // FIXME
    pub flavor: Option<String>,
}

impl RootContext {
    pub fn load(
        RootAttribute {
            trait_uses,
            // TODO: 利用
            flavor: _,
        }: RootAttribute,
    ) -> Self {
        // TODO: default 値は flavor 側に置く
        let method = default_method();
        let hook_targets = HookTargetSwitch {
            question: true,
            return_: true,
            tail_expr: true,
        };
        let tail_expr_idents = vec!["Err".to_string()];
        let result_types = vec!["Result".to_string()];
        let hook_in_macros = true;
        let bindings = HashMap::new();

        // TODO: flavor を読み込む

        Self {
            trait_uses,
            method,
            hook_targets,
            tail_expr_idents,
            result_types,
            hook_in_macros,
            bindings,
        }
    }
}

// TODO: 以下2つは flavor に移動する

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

/*
pub fn hook_method() -> TokenStream {
    parse_quote! {
        .hook(|| {
            $hooq_meta
        })
    }
}
*/
