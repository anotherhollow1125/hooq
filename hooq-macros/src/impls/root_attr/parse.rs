use proc_macro2::TokenStream;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{Expr, ExprLit, Lit, Meta, MetaList, MetaNameValue, Path, Token};

use crate::impls::root_attr::RootAttribute;

impl Parse for RootAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut trait_uses = Vec::new();
        let mut use_hook_method = false;

        for meta in input.parse_terminated(Meta::parse, Token![,])? {
            match meta {
                Meta::Path(path) => parse_meta_path(path, &mut use_hook_method)?,
                Meta::List(meta_list) => {
                    parse_meta_list(meta_list, &mut trait_uses, &mut use_hook_method)?
                }
                Meta::NameValue(meta_name_value) => {
                    parse_name_value(meta_name_value, &mut use_hook_method)?
                }
            }
        }

        Ok(RootAttribute {
            trait_uses,
            // TODO: flavor 指定を取り込むようにする
            flavor: None,
            use_hook_method,
        })
    }
}

const ROOT_ATTRIBUTE_ERROR_MESSAGE: &str = r#"expected attribute formats are below:

- trait_use(...)
- hook(...)
- hook = true or false
- preset = "..."
"#;

fn parse_meta_path(input: Path, use_hook_method: &mut bool) -> syn::Result<()> {
    match input.get_ident() {
        Some(ident) if ident == "hook" => {
            *use_hook_method = true;
            Ok(())
        }
        _ => Err(syn::Error::new_spanned(input, ROOT_ATTRIBUTE_ERROR_MESSAGE)),
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
    use_hook_method: &mut bool,
) -> syn::Result<()> {
    match path {
        p if p.is_ident("trait_use") => {
            trait_uses.extend(get_paths(tokens)?);

            Ok(())
        }
        p if p.is_ident("hook") => {
            *use_hook_method = true;
            trait_uses.extend(get_paths(tokens)?);

            Ok(())
        }
        p => Err(syn::Error::new_spanned(p, ROOT_ATTRIBUTE_ERROR_MESSAGE)),
    }
}

fn parse_name_value(input: MetaNameValue, use_hook_method: &mut bool) -> syn::Result<()> {
    match input {
        MetaNameValue {
            path,
            value:
                Expr::Lit(ExprLit {
                    lit: Lit::Bool(lit_bool),
                    ..
                }),
            ..
        } if path.is_ident("hook") => {
            *use_hook_method = lit_bool.value();

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
        } if path.is_ident("preset") && lit_str.value() == "hook" => {
            *use_hook_method = true;

            Ok(())
        }
        else_ => Err(syn::Error::new_spanned(else_, ROOT_ATTRIBUTE_ERROR_MESSAGE)),
    }
}
