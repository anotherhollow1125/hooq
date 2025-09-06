use std::collections::HashMap;
use std::rc::Rc;

use proc_macro2::{Span, TokenStream};
use syn::{Expr, Path};

use crate::impls::flavor::Flavor;
use crate::impls::flavor::toml_load::{FlavorTable, HooqToml};

pub fn update_flavors(
    flavors: &mut HashMap<String, Flavor>,
    hooq_toml: HooqToml,
) -> syn::Result<()> {
    update_flavor_inner(flavors, hooq_toml.flavors, &Flavor::default())?;

    Ok(())
}

fn update_flavor_inner(
    flavors: &mut HashMap<String, Flavor>,
    flavor_tables: HashMap<String, FlavorTable>,
    base_flavor: &Flavor,
) -> syn::Result<()> {
    for (
        flavor_name,
        FlavorTable {
            trait_uses,
            method,
            hook_targets,
            tail_expr_idents,
            result_types,
            hook_in_macros,
            bindings,
            sub_flavors,
        },
    ) in flavor_tables
    {
        if flavor_name.is_empty() {
            return Err(syn::Error::new(
                Span::call_site(),
                "flavor name must not be empty",
            ));
        }

        // ↑↓ ややこしいw

        if flavor_name == "empty" {
            return Err(syn::Error::new(
                Span::call_site(),
                r#"special flavor `empty` can't be overriden"#,
            ));
        }

        // NOTE:
        // or_insert_with(|| base_flavor.clone()) で
        // デフォルト値としているが、
        // 今後複数ファイルに分割して読み込めるようにする場合
        // この方法だと親flavorが後から別ファイルで更新された時に
        // それが反映されなくなってしまう
        //
        // 本来はツリー全部を残して取得時に毎回作り直すのが理想である
        //
        // ただ分割機能は検討中であるためYAGNI原則に従い
        // 現在はこのような素直な実装としておく
        let flavor = flavors
            .entry(flavor_name)
            .or_insert_with(|| base_flavor.clone());

        let trait_uses = trait_uses
            .into_iter()
            .map(|path| syn::parse_str::<Path>(&path))
            .collect::<syn::Result<Vec<_>>>()?;
        flavor.trait_uses.extend(trait_uses);

        if let Some(method) = method {
            let method_stream = syn::parse_str::<TokenStream>(&method)?;

            flavor.method = method_stream;
        }

        if let Some(hook_targets) = hook_targets {
            let hook_target_switch = hook_targets.try_into().map_err(|e| {
                syn::Error::new(
                    Span::call_site(),
                    format!(
                        r#"invalid hook_targets value. got: {e}
expected: "?", "return", "tail_expr""#,
                    ),
                )
            })?;

            flavor.hook_targets = hook_target_switch;
        }

        if let Some(tail_expr_idents) = tail_expr_idents {
            flavor.tail_expr_idents = tail_expr_idents;
        }

        if let Some(result_types) = result_types {
            flavor.result_types = result_types;
        }

        if let Some(hook_in_macros) = hook_in_macros {
            flavor.hook_in_macros = hook_in_macros;
        }

        let bindings = bindings
            .into_iter()
            .map(|(k, v)| {
                let v = syn::parse_str::<Expr>(&v)?;

                Ok((k, Rc::new(v)))
            })
            .collect::<syn::Result<Vec<_>>>()?;
        flavor.bindings.extend(bindings);

        if !sub_flavors.is_empty() {
            let base_flavor = flavor.clone();
            update_flavor_inner(flavors, sub_flavors, &base_flavor)?;
        }
    }

    Ok(())
}
