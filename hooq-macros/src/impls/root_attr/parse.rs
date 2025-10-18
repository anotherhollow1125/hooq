use proc_macro2::TokenStream;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{Expr, ExprLit, Lit, Meta, MetaList, MetaNameValue, Path, Token};

use crate::impls::flavor::FlavorStore;
use crate::impls::root_attr::RootAttribute;

impl Parse for RootAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let span = input.span();
        let mut trait_uses = Vec::new();
        let mut flavor = None;

        for meta in input.parse_terminated(Meta::parse, Token![,])? {
            match meta {
                Meta::Path(path) => parse_meta_path(path, &mut flavor)?,
                Meta::List(meta_list) => parse_meta_list(meta_list, &mut trait_uses)?,
                Meta::NameValue(meta_name_value) => parse_name_value(meta_name_value, &mut flavor)?,
            }
        }

        Ok(RootAttribute {
            trait_uses,
            flavor,
            span,
        })
    }
}

fn root_attribute_error_message() -> String {
    let flavor_names = FlavorStore::with_hooq_toml()
        .map(|store| {
            store
                .all_flavor_names()
                .into_iter()
                .map(|name| format!("  - {name}"))
                .collect::<Vec<_>>()
                .join("\n")
        })
        .unwrap_or_default();

    format!(
        r#"expected attribute formats are below:

- trait_use(...)
- FLAVOR_NAME
{flavor_names}
- flavor = "FLAVOR_NAME"
"#
    )
}

fn parse_meta_path(input: Path, flavor: &mut Option<Vec<String>>) -> syn::Result<()> {
    let idents = input
        .segments
        .into_iter()
        .map(|seg| {
            if !seg.arguments.is_none() {
                return Err(syn::Error::new_spanned(
                    seg,
                    "path with generic arguments is not supported here",
                ));
            }

            let flavor_name = seg.ident.to_string();

            if flavor_name.is_empty() {
                return Err(syn::Error::new_spanned(
                    seg,
                    "flavor names must not include empty one",
                ));
            }

            Ok(flavor_name)
        })
        .collect::<syn::Result<Vec<_>>>()?;

    if !idents.is_empty() {
        *flavor = Some(idents);
    }

    Ok(())
}

fn get_paths(tokens: TokenStream) -> syn::Result<Punctuated<Path, Comma>> {
    struct Paths(Punctuated<Path, Comma>);

    impl Parse for Paths {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let paths = input.parse_terminated(Path::parse, Token![,])?;
            Ok(Self(paths))
        }
    }

    let paths = syn::parse2::<Paths>(tokens.clone())?.0;

    Ok(paths)
}

fn parse_meta_list(
    MetaList { path, tokens, .. }: MetaList,
    trait_uses: &mut Vec<Path>,
) -> syn::Result<()> {
    match path {
        p if p.is_ident("trait_use") => {
            trait_uses.extend(get_paths(tokens)?);

            Ok(())
        }
        p => Err(syn::Error::new_spanned(p, root_attribute_error_message())),
    }
}

fn parse_name_value(input: MetaNameValue, flavor: &mut Option<Vec<String>>) -> syn::Result<()> {
    match input {
        MetaNameValue {
            path,
            value:
                Expr::Lit(ExprLit {
                    lit: Lit::Str(lit_str),
                    ..
                }),
            ..
        } if path.is_ident("flavor") => {
            let span = lit_str.span();

            let flavor_names = lit_str
                .value()
                .split('.')
                .flat_map(|s| s.split("::"))
                .map(|s| {
                    if s.is_empty() {
                        return Err(syn::Error::new(
                            span,
                            "flavor names must not include empty one",
                        ));
                    }

                    Ok(s.to_string())
                })
                .collect::<syn::Result<Vec<_>>>()?;

            *flavor = Some(flavor_names);

            Ok(())
        }
        else_ => Err(syn::Error::new_spanned(
            else_,
            root_attribute_error_message(),
        )),
    }
}
