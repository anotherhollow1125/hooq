use proc_macro2::TokenStream;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{Expr, ExprLit, Lit, Meta, MetaList, MetaNameValue, Path, Token};

use crate::impls::root_attr::HooqRootOption;

impl Parse for HooqRootOption {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut trait_uses = Vec::new();
        let mut is_custom = false;

        for meta in input.parse_terminated(Meta::parse, Token![,])? {
            match meta {
                Meta::Path(path) => parse_meta_path(path, &mut is_custom)?,
                Meta::List(meta_list) => {
                    parse_meta_list(meta_list, &mut trait_uses, &mut is_custom)?
                }
                Meta::NameValue(meta_name_value) => {
                    parse_name_value(meta_name_value, &mut is_custom)?
                }
            }
        }

        Ok(HooqRootOption {
            trait_uses,
            is_custom,
        })
    }
}

const ATTRIBUTE_ERROR_MESSAGE: &str = r#"Expected attribute formats are below:

- trait_use(...)
- custom(...)
- custom = true or false
- preset = "..."
"#;

fn parse_meta_path(input: Path, is_custom: &mut bool) -> syn::Result<()> {
    match input.get_ident() {
        Some(ident) if ident == "custom" => {
            *is_custom = true;
            Ok(())
        }
        _ => Err(syn::Error::new_spanned(input, ATTRIBUTE_ERROR_MESSAGE)),
    }
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
    is_custom: &mut bool,
) -> syn::Result<()> {
    match path {
        p if p.is_ident("trait_use") => {
            trait_uses.extend(get_paths(tokens)?);

            Ok(())
        }
        p if p.is_ident("custom") => {
            *is_custom = true;
            trait_uses.extend(get_paths(tokens)?);

            Ok(())
        }
        p => Err(syn::Error::new_spanned(p, ATTRIBUTE_ERROR_MESSAGE)),
    }
}

fn parse_name_value(input: MetaNameValue, is_custom: &mut bool) -> syn::Result<()> {
    match input {
        MetaNameValue {
            path,
            value:
                Expr::Lit(ExprLit {
                    lit: Lit::Bool(lit_bool),
                    ..
                }),
            ..
        } if path.is_ident("custom") => {
            *is_custom = lit_bool.value();

            Ok(())
        }
        MetaNameValue {
            path,
            value:
                Expr::Lit(ExprLit {
                    lit: Lit::Str(lit_str),
                    ..
                }),
            ..
        } if path.is_ident("preset") && lit_str.value() == "custom" => {
            *is_custom = true;

            Ok(())
        }
        else_ => Err(syn::Error::new_spanned(else_, ATTRIBUTE_ERROR_MESSAGE)),
    }
}
