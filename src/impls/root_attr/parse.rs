use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{Meta, MetaList, MetaNameValue, Path, Token};

use crate::impls::root_attr::HooqRootOption;

impl Parse for HooqRootOption {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut trait_uses = Vec::new();

        for meta in input.parse_terminated(Meta::parse, Token![,])? {
            match meta {
                Meta::Path(path) => parse_meta_path(path)?,
                Meta::List(meta_list) => parse_meta_list(meta_list, &mut trait_uses)?,
                Meta::NameValue(meta_name_value) => parse_name_value(meta_name_value)?,
            }
        }

        Ok(HooqRootOption { trait_uses })
    }
}

fn parse_meta_path(input: Path) -> syn::Result<()> {
    // error now.
    Err(syn::Error::new_spanned(
        input,
        "Expected trait_use(...) or custom(...) attribute format",
    ))
}

fn parse_meta_list(input: MetaList, trait_uses: &mut Vec<Path>) -> syn::Result<()> {
    match input {
        MetaList { path, tokens, .. } if path.is_ident("trait_use") || path.is_ident("custom") => {
            struct Paths(Punctuated<Path, Comma>);

            impl Parse for Paths {
                fn parse(input: ParseStream) -> syn::Result<Self> {
                    let paths = input.parse_terminated(Path::parse, Token![,])?;
                    Ok(Self(paths))
                }
            }

            let paths = syn::parse2::<Paths>(tokens.clone())?.0;
            trait_uses.extend(paths);

            Ok(())
        }
        _ => Err(syn::Error::new_spanned(
            input,
            "Expected trait_use(...) or custom(...) attribute format",
        )),
    }
}

fn parse_name_value(input: MetaNameValue) -> syn::Result<()> {
    // error now.
    Err(syn::Error::new_spanned(
        input,
        "Expected trait_use(...) or custom(...) attribute format",
    ))
}
