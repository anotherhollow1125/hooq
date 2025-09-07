use std::collections::HashMap;
use std::rc::Rc;

use proc_macro2::{Span, TokenStream};
use syn::{Expr, Path};

use crate::impls::flavor::{Flavor, FlavorStore};
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
    pub span: Span,
}

impl RootContext {
    pub fn load(
        RootAttribute {
            mut trait_uses,
            flavor,
            span,
        }: RootAttribute,
    ) -> syn::Result<Self> {
        // NOTE:
        // default への上書きが存在する可能性があるので
        // 未指定時でもあくまでも FlavorStore から取得する必要あり
        let flavor = flavor.unwrap_or(vec!["default".to_string()]);

        let flavor_store = FlavorStore::with_hooq_toml();

        let Flavor {
            trait_uses: trait_uses_of_flavor,
            method,
            hook_targets,
            tail_expr_idents,
            result_types,
            hook_in_macros,
            bindings,
            sub_flavors: _,
        } = flavor_store.get_flavor(&flavor).ok_or_else(|| {
            syn::Error::new(
                span,
                format!(
                    "flavor `{}` is not found. available flavors:
{}",
                    flavor.join("::"),
                    flavor_store
                        .all_flavor_names()
                        .into_iter()
                        .map(|name| format!("  - {name}"))
                        .collect::<Vec<_>>()
                        .join("\n")
                ),
            )
        })?;

        trait_uses.extend(trait_uses_of_flavor);

        Ok(Self {
            trait_uses,
            method,
            hook_targets,
            tail_expr_idents,
            result_types,
            hook_in_macros,
            bindings,
        })
    }
}
