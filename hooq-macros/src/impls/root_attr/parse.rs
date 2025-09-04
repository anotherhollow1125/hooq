use proc_macro2::TokenStream;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{Expr, ExprLit, Lit, Meta, MetaList, MetaNameValue, Path, Token};

use crate::impls::root_attr::RootAttribute;

impl Parse for RootAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut trait_uses = Vec::new();
        let mut flavor = None;

        for meta in input.parse_terminated(Meta::parse, Token![,])? {
            match meta {
                Meta::Path(path) => parse_meta_path(path, &mut flavor)?,
                Meta::List(meta_list) => parse_meta_list(meta_list, &mut trait_uses)?,
                Meta::NameValue(meta_name_value) => parse_name_value(meta_name_value, &mut flavor)?,
            }
        }

        Ok(RootAttribute { trait_uses, flavor })
    }
}

const ROOT_ATTRIBUTE_ERROR_MESSAGE: &str = r#"expected attribute formats are below:

- trait_use(...)
- FLAVOR_NAME
  - hook
  - log
  - ...
- flavor = "FLAVOR_NAME"
"#;

fn parse_meta_path(input: Path, flavor: &mut Option<String>) -> syn::Result<()> {
    match input.get_ident() {
        Some(ident) => {
            *flavor = Some(ident.to_string());
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
) -> syn::Result<()> {
    match path {
        p if p.is_ident("trait_use") => {
            trait_uses.extend(get_paths(tokens)?);

            Ok(())
        }
        p => Err(syn::Error::new_spanned(p, ROOT_ATTRIBUTE_ERROR_MESSAGE)),
    }
}

fn parse_name_value(input: MetaNameValue, flavor: &mut Option<String>) -> syn::Result<()> {
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
            *flavor = Some(lit_str.value());

            Ok(())
        }
        else_ => Err(syn::Error::new_spanned(else_, ROOT_ATTRIBUTE_ERROR_MESSAGE)),
    }
}
