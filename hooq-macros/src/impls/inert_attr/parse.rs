use std::collections::HashMap;

use proc_macro2::Span;
use syn::parse::Parse;
use syn::spanned::Spanned;
use syn::{
    Attribute, Expr, ExprLit, ExprPath, Lit, LitBool, LitStr, Meta, MetaList, MetaNameValue, Path,
    Token, parse_quote,
};

use crate::impls::flavor::{Flavor, FlavorPath, FlavorStore};
use crate::impls::inert_attr::InertAttribute;
use crate::impls::inert_attr::context::HookTargetSwitch;
use crate::impls::method::Method;

fn get_binding_name(path: &Path) -> syn::Result<Option<String>> {
    let mut segs = path.segments.iter();

    let Some(maybe_hooq) = segs.next() else {
        return Ok(None);
    };

    if maybe_hooq.ident != "hooq" {
        return Ok(None);
    }

    let maybe_binding = segs.next();

    if segs.next().is_some() {
        return Err(syn::Error::new_spanned(
            path,
            "invalid hooq attribute format. expected: hooq::xxx, hooq::xxx(...) or hooq::xxx = ...",
        ));
    }

    Ok(maybe_binding.map(|s| s.ident.to_string()))
}

struct Strings(Vec<String>);

impl Parse for Strings {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Strings(
            input
                .parse_terminated(<LitStr as Parse>::parse, Token![,])?
                .into_iter()
                .map(|lit| lit.value())
                .collect::<Vec<_>>(),
        ))
    }
}

impl Strings {
    fn split_include_or_not_include(self) -> (Vec<String>, Vec<String>) {
        let mut includes = Vec::new();
        let mut not_includes = Vec::new();

        for s in self.0 {
            if let Some(stripped) = s.strip_prefix('!') {
                not_includes.push(stripped.to_string());
            } else {
                includes.push(s);
            }
        }

        (includes, not_includes)
    }
}

const INERT_ATTRIBUTE_ERROR_MESSAGE: &str = r#"expected attribute formats are below:

- #[hooq::method(/* e.g. `.method(...)` */)]
- #[hooq::hook_targets(/* e.g. `"return", "?"` */)]
- #[hooq::tail_expr_idents(/* e.g. `Err` */)]
- #[hooq::ignore_tail_expr_idents(/* e.g. `Ok` */)]
- #[hooq::result_types(/* e.g. `XxxResult` */)]
- #[hooq::hook_in_macros(/* `true` or `false` */)]
- #[hooq::binding(xxx = /* e.g. `42` */)] // xxx is param name
- #[hooq::xxx = /* e.g. `42` */] // alternative syntax for binding
- #[hooq::skip]
- #[hooq::skip_all]

or like #[hooq::SETTING = FLAVOR_NAME] format where
- SETTING is one of:
  - flavor
  - method
  - hook_targets
  - tail_expr_idents // ignore_tail_expr_idents is included here
  - result_types
  - hook_in_macros
  - bindings
- FLAVOR_NAME is a flavor name as path or string
This format overrides the corresponding setting with the one defined in the specified flavor."
"#;

fn get_flavor_path(value: &Expr, setting_name: &str) -> syn::Result<FlavorPath> {
    match value {
        Expr::Path(ExprPath { path, .. }) => Ok(path
            .try_into()
            .map_err(|e| syn::Error::new_spanned(path, e))?),
        Expr::Lit(ExprLit {
            lit: Lit::Str(lit), ..
        }) => Ok(lit
            .value()
            .try_into()
            .map_err(|e| syn::Error::new_spanned(lit, e))?),
        _ => Err(syn::Error::new_spanned(
            value,
            format!(
                "invalid hooq::{setting_name} attribute value. expected: FLAVOR_NAME as path or string"
            ),
        )),
    }
}

fn get_flavor(span: Span, flavor_path: &FlavorPath) -> syn::Result<Flavor> {
    FlavorStore::with_hooq_toml()
        .map_err(|e| syn::Error::new(span, format!("failed to load hooq.toml: {e}")))?
        .get_flavor(flavor_path)
        .map_err(|e| syn::Error::new(span, e))
}

pub fn extract_hooq_info_from_attrs(attrs: &mut Vec<Attribute>) -> syn::Result<InertAttribute> {
    // #[hooq::flavor = FLAVOR_NAME]
    let hooq_flavor = parse_quote!(hooq::flavor);
    // #[hooq::method(...)] or #[hooq::method = ...]
    let hooq_method = parse_quote!(hooq::method);
    // #[hooq::hook_targets("return", "?", ...)] or #[hooq::hook_targets = ...]
    let hooq_hook_targets = parse_quote!(hooq::hook_targets);
    // #[hooq::tail_expr_idents("Err", ...)] or #[hooq::tail_expr_idents = ...]
    let hooq_tail_expr_idents = parse_quote!(hooq::tail_expr_idents);
    // #[hooq::ignore_tail_expr_idents("Ok", ...)]
    let hooq_ignore_tail_expr_idents = parse_quote!(hooq::ignore_tail_expr_idents);
    // #[hooq::result_types("XxxResult", ...)] or #[hooq::result_types = ...]
    let hooq_result_types = parse_quote!(hooq::result_types);
    // #[hooq::hook_in_macros(true | false)] or #[hooq::hook_in_macros = ...]
    let hooq_hook_in_macros = parse_quote!(hooq::hook_in_macros);
    // #[hooq::bindings = FLAVOR_NAME]
    let hooq_bindings = parse_quote!(hooq::bindings);
    // #[hooq::binding(xxx = ...)]
    let hooq_binding = parse_quote!(hooq::binding);
    // #[hooq::var(xxx = ...)]
    let hooq_var = parse_quote!(hooq::var);
    // #[hooq::skip]
    let hooq_skip = parse_quote!(hooq::skip);
    // #[hooq::skip_all]
    let hooq_skip_all = parse_quote!(hooq::skip_all);

    let mut method: Option<Method> = None;
    let mut hook_targets: Option<HookTargetSwitch> = None;
    let mut tail_expr_idents: Option<Vec<String>> = None;
    let mut ignore_tail_expr_idents: Option<Vec<String>> = None;
    let mut result_types: Option<Vec<String>> = None;
    let mut hook_in_macros: Option<bool> = None;
    let mut bindings: HashMap<String, Expr> = HashMap::new();
    let mut is_skipped = false;
    let mut is_skipped_all = false;

    let mut keeps = Vec::with_capacity(attrs.len());
    for attr in attrs.iter() {
        match &attr.meta {
            // flavor
            Meta::NameValue(MetaNameValue { path, value, .. }) if path == &hooq_flavor => {
                let flavor_path: FlavorPath = get_flavor_path(value, "flavor")?;
                let Flavor {
                    trait_uses: _,
                    method: flavor_method,
                    hook_targets: flavor_hook_targets,
                    tail_expr_idents: flavor_tail_expr_idents,
                    ignore_tail_expr_idents: flavor_ignore_tail_expr_idents,
                    result_types: flavor_result_types,
                    hook_in_macros: flavor_hook_in_macros,
                    bindings: flavor_bindings,
                    sub_flavors: _,
                } = get_flavor(path.span(), &flavor_path)?;

                // override settings by the flavor settings
                method = Some(flavor_method);
                hook_targets = Some(flavor_hook_targets);
                tail_expr_idents = Some(flavor_tail_expr_idents);
                ignore_tail_expr_idents = Some(flavor_ignore_tail_expr_idents);
                result_types = Some(flavor_result_types);
                hook_in_macros = Some(flavor_hook_in_macros);
                for (k, v) in flavor_bindings.iter() {
                    bindings.insert(k.clone(), v.as_ref().clone());
                }

                keeps.push(false);
            }

            // method
            Meta::List(MetaList { path, tokens, .. }) if path == &hooq_method => {
                method = Some(tokens.clone().try_into()?);
                keeps.push(false);
            }
            Meta::NameValue(MetaNameValue { path, value, .. }) if path == &hooq_method => {
                let flavor_path: FlavorPath = get_flavor_path(value, "method")?;
                method = Some(get_flavor(path.span(), &flavor_path)?.method);

                keeps.push(false);
            }

            // hook_targets
            Meta::List(MetaList { path, tokens, .. }) if path == &hooq_hook_targets => {
                let strings = syn::parse2::<Strings>(tokens.clone())?;
                match HookTargetSwitch::try_from(strings.0) {
                    Ok(switch) => {
                        hook_targets = Some(switch);
                        keeps.push(false);
                    }
                    Err(e) => {
                        return Err(syn::Error::new_spanned(
                            path,
                            format!(
                                r#"invalid hooq::hook_targets attribute value. got: {e}
expected: "?", "return", "tail_expr""#,
                            ),
                        ));
                    }
                }
            }
            Meta::NameValue(MetaNameValue { path, value, .. }) if path == &hooq_hook_targets => {
                let flavor_path: FlavorPath = get_flavor_path(value, "hook_targets")?;
                hook_targets = Some(get_flavor(path.span(), &flavor_path)?.hook_targets);

                keeps.push(false);
            }

            // tail_expr_idents or ignore_tail_expr_idents
            Meta::List(MetaList { path, tokens, .. }) if path == &hooq_tail_expr_idents => {
                let strings = syn::parse2::<Strings>(tokens.clone())?;

                let (includes, not_includes) = strings.split_include_or_not_include();

                tail_expr_idents = Some(includes);
                ignore_tail_expr_idents = match ignore_tail_expr_idents {
                    // すでに存在する場合は追加
                    Some(v) => Some([v, not_includes].concat()),
                    // 空ではないとき、指定が行われているので上書き
                    None if !not_includes.is_empty() => Some(not_includes),
                    // 空であるときは、ignoreに当たる指定が行われていないのでそのまま (※)
                    None => None,
                };
                keeps.push(false);
            }
            Meta::NameValue(MetaNameValue { path, value, .. })
                if path == &hooq_tail_expr_idents =>
            {
                let flavor_path: FlavorPath = get_flavor_path(value, "tail_expr_idents")?;
                let flavor = get_flavor(path.span(), &flavor_path)?;

                // flavor指定では ["Err", "!Ok"] のようなパターンに対応できない("Err" のみ適用されるみたいになりかねない)ので、
                // tail_expr_idents による指定で両方を兼ねることにする
                tail_expr_idents = Some(flavor.tail_expr_idents.clone());
                ignore_tail_expr_idents = Some(flavor.ignore_tail_expr_idents.clone());

                keeps.push(false);
            }

            // ignore_tail_expr_idents
            // このフィールドのみ、tail_expr_idents と ignore_tail_expr_idents の両方から設定可能
            Meta::List(MetaList { path, tokens, .. }) if path == &hooq_ignore_tail_expr_idents => {
                let strings = syn::parse2::<Strings>(tokens.clone())?;

                ignore_tail_expr_idents = match ignore_tail_expr_idents {
                    // すでに存在する場合は追加
                    Some(v) => Some([v, strings.0].concat()),
                    // (※) とは異なり、こちらは空配列であっても指定のため上書き
                    None => Some(strings.0),
                };
                keeps.push(false);
            }
            // ignore_tail_expr_idents については `#[hooq::ignore_tail_expr_idents = ...]` 形式は非対応とする
            Meta::NameValue(MetaNameValue { path, .. })
                if path == &hooq_ignore_tail_expr_idents =>
            {
                return Err(syn::Error::new_spanned(
                    path,
                    "you can't use #[hooq::ignore_tail_expr_idents = FLAVOR_NAME] format.
please use #[hooq::tail_expr_idents = FLAVOR_NAME] format instead.
this format can set both tail_expr_idents and ignore_tail_expr_idents fields by the flavor settings.",
                ));
            }

            // result_types
            Meta::List(MetaList { path, tokens, .. }) if path == &hooq_result_types => {
                let strings = syn::parse2::<Strings>(tokens.clone())?;
                result_types = Some(strings.0);
                keeps.push(false);
            }
            Meta::NameValue(MetaNameValue { path, value, .. }) if path == &hooq_result_types => {
                let flavor_path: FlavorPath = get_flavor_path(value, "result_types")?;
                result_types = Some(get_flavor(path.span(), &flavor_path)?.result_types);

                keeps.push(false);
            }

            // hook_in_macros
            Meta::List(MetaList { path, tokens, .. }) if path == &hooq_hook_in_macros => {
                let hook = syn::parse2::<LitBool>(tokens.clone())?;
                hook_in_macros = Some(hook.value());
                keeps.push(false);
            }
            Meta::NameValue(MetaNameValue { path, value, .. }) if path == &hooq_hook_in_macros => {
                let flavor_path: FlavorPath = get_flavor_path(value, "hook_in_macros")?;
                hook_in_macros = Some(get_flavor(path.span(), &flavor_path)?.hook_in_macros);

                keeps.push(false);
            }

            // set bindings by the flavor setting
            Meta::NameValue(MetaNameValue { path, value, .. }) if path == &hooq_bindings => {
                let flavor_path: FlavorPath = get_flavor_path(value, "bindings")?;
                let flavor = get_flavor(path.span(), &flavor_path)?;

                for (k, v) in flavor.bindings.iter() {
                    bindings.insert(k.clone(), v.as_ref().clone());
                }

                keeps.push(false);
            }

            // is_skipped
            Meta::Path(p) if p == &hooq_skip => {
                is_skipped = true;
                keeps.push(false);
            }
            // is_skipped_all
            Meta::Path(p) if p == &hooq_skip_all => {
                is_skipped_all = true;
                keeps.push(false);
            }
            // 残りは bindings
            Meta::List(MetaList { path, tokens, .. })
                if path == &hooq_binding || path == &hooq_var =>
            {
                let MetaNameValue { path, value, .. } =
                    syn::parse2::<MetaNameValue>(tokens.clone())?;

                let Some(binding) = path.get_ident() else {
                    return Err(syn::Error::new_spanned(
                        path,
                        "invalid hooq::binding or hooq::var attribute format. expected: hooq::binding(name = value) or hooq::var(name = value)",
                    ));
                };

                bindings.insert(binding.to_string(), value.clone());

                keeps.push(false);
            }
            Meta::NameValue(MetaNameValue { path, value, .. }) => {
                if let Some(binding) = get_binding_name(path)? {
                    bindings.insert(binding, value.clone());
                    keeps.push(false);
                } else {
                    keeps.push(true);
                }
            }

            // 以降は hooq::xxx という形式になっていたらエラーとした
            Meta::List(MetaList { path, .. }) => {
                if get_binding_name(path)?.is_some() {
                    return Err(syn::Error::new_spanned(path, INERT_ATTRIBUTE_ERROR_MESSAGE));
                } else {
                    keeps.push(true);
                };
            }
            Meta::Path(path) => {
                if get_binding_name(path)?.is_some() {
                    return Err(syn::Error::new_spanned(path, INERT_ATTRIBUTE_ERROR_MESSAGE));
                } else {
                    keeps.push(true);
                };
            }
        }
    }

    // ref: https://doc.rust-lang.org/alloc/vec/struct.Vec.html#method.retain
    let mut keeps_iter = keeps.iter();
    attrs.retain(|_| *keeps_iter.next().unwrap());

    Ok(InertAttribute {
        method,
        hook_targets,
        tail_expr_idents,
        ignore_tail_expr_idents,
        result_types,
        hook_in_macros,
        bindings,
        is_skipped,
        is_skipped_all,
    })
}
