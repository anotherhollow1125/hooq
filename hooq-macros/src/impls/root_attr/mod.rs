use std::collections::HashMap;
use std::rc::Rc;

use proc_macro2::TokenStream;
use syn::{Expr, Path, parse_quote};

use crate::impls::flavor::{FLAVORS, Flavor};
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
    pub flavor: Option<Vec<String>>,
}

impl RootContext {
    pub fn load(
        RootAttribute {
            mut trait_uses,
            flavor,
        }: RootAttribute,
    ) -> Self {
        // NOTE:
        // default への上書きが存在する可能性があるので
        // 未指定時でもあくまでも FlavorStore から取得する必要あり
        //
        // この後の (★) の `unwrap_or_default` に関連s
        let flavor = flavor.unwrap_or(vec!["default".to_string()]);

        todo!();

        let Flavor {
            trait_uses: trait_uses_of_flavor,
            method,
            hook_targets,
            tail_expr_idents,
            result_types,
            hook_in_macros,
            bindings,
            sub_flavors: _,
        } = FLAVORS.with(|flavors| {
            flavors
                .get_flavor(&flavor)
                // (★) Unreachable のはず
                .unwrap_or_default()
        });

        trait_uses.extend(trait_uses_of_flavor);

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
