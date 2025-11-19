use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::Parse;
use syn::parse::discouraged::Speculative;
use syn::{Expr, Token};

use crate::impls::utils::source_excerpt::{
    ExcerptSetting, PrettyStrSettings, TruncateLitStrSetting, into_excerpted_line_pretty_str,
    into_one_line_pretty_str, into_pretty_str, truncate_lit_str as truncate_lit_str_inner,
};

enum Padding {
    Top,
    Bottom,
    Both,
}

// boolだと設定値と混同しやすいので
// 一旦存在を表す Option<()> 型にしておく
struct ExcerptMacroArgs {
    max_excerpted_line_num: Option<usize>, // @excerpt_line = 3,
    truncate_lit_str_max_len: Option<Option<usize>>, // @truncate_str = 10, or @truncate_str
    show_line_num: Option<()>,             // @show_line_num,
    padding: Option<Padding>,              // @padding = "top", "bottom" or "both"
    expr: Expr,
}

impl Parse for ExcerptMacroArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut max_excerpted_line_num = None;
        let mut truncate_lit_str_max_len = None;
        let mut show_line_num = None;
        let mut padding = None;

        // @opt = val
        struct Opt {
            name: String,
            val: Option<syn::Lit>,
        }

        impl Parse for Opt {
            fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
                let _ = input.parse::<syn::Token![@]>()?;
                let name_ident: syn::Ident = input.parse()?;

                if input.peek(Token![,]) || input.peek(Token![;]) {
                    return Ok(Opt {
                        name: name_ident.to_string(),
                        val: None,
                    });
                }

                let _ = input.parse::<syn::Token![=]>()?;
                let val: syn::Lit = input.parse()?;

                Ok(Opt {
                    name: name_ident.to_string(),
                    val: Some(val),
                })
            }
        }

        while !input.is_empty() {
            if input.peek(Token![;]) {
                let _ = input.parse::<Token![;]>()?;
                break;
            }

            let ahead = input.fork();
            let Ok(opt) = ahead.parse::<Opt>() else {
                break;
            };

            input.advance_to(&ahead);

            match opt.name.as_str() {
                "excerpt_line" => {
                    let Some(line_num) = opt.val else {
                        return Err(syn::Error::new_spanned(
                            &opt.name,
                            "@excerpt_line requires a value",
                        ));
                    };

                    let line_num = match line_num {
                        syn::Lit::Int(lit_int) => lit_int.base10_parse::<usize>()?,
                        _ => {
                            return Err(syn::Error::new_spanned(
                                line_num,
                                "@excerpt_line requires an integer value",
                            ));
                        }
                    };

                    max_excerpted_line_num = Some(line_num);
                }
                "truncate_str" => {
                    let val_opt = match opt.val {
                        Some(lit) => match lit {
                            syn::Lit::Int(lit_int) => Some(lit_int.base10_parse::<usize>()?),
                            _ => {
                                return Err(syn::Error::new_spanned(
                                    lit,
                                    "@truncate_str requires an integer value",
                                ));
                            }
                        },
                        None => None,
                    };

                    truncate_lit_str_max_len = Some(val_opt);
                }
                "show_line_num" => {
                    show_line_num = Some(());
                }
                "padding" => match opt.val {
                    Some(val) => match val {
                        syn::Lit::Str(lit_str) => {
                            let s = lit_str.value();
                            match s.as_str() {
                                "top" => {
                                    padding = Some(Padding::Top);
                                }
                                "bottom" => {
                                    padding = Some(Padding::Bottom);
                                }
                                "both" => {
                                    padding = Some(Padding::Both);
                                }
                                _ => {
                                    return Err(syn::Error::new_spanned(
                                        lit_str,
                                        r#"@padding requires "top", "bottom" or "both""#,
                                    ));
                                }
                            }
                        }
                        _ => {
                            return Err(syn::Error::new_spanned(
                                val,
                                "@padding requires a string value",
                            ));
                        }
                    },
                    None => {
                        padding = Some(Padding::Both);
                    }
                },
                _ => {
                    return Err(syn::Error::new_spanned(
                        opt.name,
                        "unknown option\n available options:\n- @excerpt_line\n- @truncate_str\n- @show_line_num\n- @padding",
                    ));
                }
            }

            if input.peek(Token![,]) {
                let _ = input.parse::<Token![,]>()?;
            }
        }

        let expr: Expr = input.parse()?;

        Ok(ExcerptMacroArgs {
            max_excerpted_line_num,
            truncate_lit_str_max_len,
            show_line_num,
            padding,
            expr,
        })
    }
}

pub fn truncate_lit_str(ts: TokenStream) -> syn::Result<TokenStream> {
    let ExcerptMacroArgs {
        truncate_lit_str_max_len,
        expr,
        ..
    } = syn::parse2::<ExcerptMacroArgs>(ts)?;

    truncate_lit_str_inner(expr, truncate_lit_str_max_len.flatten())
}

pub fn pretty_stringify(ts: TokenStream) -> syn::Result<TokenStream> {
    let ExcerptMacroArgs {
        show_line_num,
        padding,
        expr,
        ..
    } = syn::parse2::<ExcerptMacroArgs>(ts)?;

    let mut settings = PrettyStrSettings {
        show_line_num: false,
        top_padding: false,
        bottom_padding: false,
    };

    if show_line_num.is_some() {
        settings.show_line_num = true;
    }

    match padding {
        Some(Padding::Top) => {
            settings.top_padding = true;
        }
        Some(Padding::Bottom) => {
            settings.bottom_padding = true;
        }
        Some(Padding::Both) => {
            settings.top_padding = true;
            settings.bottom_padding = true;
        }
        None => (),
    }

    Ok(into_pretty_str(expr, settings)
        .to_string()
        .into_token_stream())
}

pub fn one_line_stringify(ts: TokenStream) -> syn::Result<TokenStream> {
    let ExcerptMacroArgs {
        truncate_lit_str_max_len,
        expr,
        ..
    } = syn::parse2::<ExcerptMacroArgs>(ts)?;

    Ok(into_one_line_pretty_str(
        expr,
        match truncate_lit_str_max_len {
            Some(val) => TruncateLitStrSetting::Truncate(val),
            None => TruncateLitStrSetting::NoTruncation,
        },
    )?
    .into_token_stream())
}

pub fn excerpted_pretty_stringify(ts: TokenStream) -> syn::Result<TokenStream> {
    let ExcerptMacroArgs {
        max_excerpted_line_num,
        truncate_lit_str_max_len,
        show_line_num,
        padding,
        expr,
    } = syn::parse2::<ExcerptMacroArgs>(ts)?;

    let mut excerpt_setting = ExcerptSetting::default();

    if let Some(line_num) = max_excerpted_line_num {
        excerpt_setting.max_excerpted_line_num = line_num;
    }

    if let Some(val) = truncate_lit_str_max_len {
        excerpt_setting.truncate_lit_str_setting = TruncateLitStrSetting::Truncate(val);
    }

    if let Some(()) = show_line_num {
        excerpt_setting.pretty_str_settings.show_line_num = true;
    }

    match padding {
        Some(Padding::Top) => {
            excerpt_setting.pretty_str_settings.top_padding = true;
        }
        Some(Padding::Bottom) => {
            excerpt_setting.pretty_str_settings.bottom_padding = true;
        }
        Some(Padding::Both) => {
            excerpt_setting.pretty_str_settings.top_padding = true;
            excerpt_setting.pretty_str_settings.bottom_padding = true;
        }
        None => (),
    }

    Ok(into_excerpted_line_pretty_str(expr, excerpt_setting)?.into_token_stream())
}
