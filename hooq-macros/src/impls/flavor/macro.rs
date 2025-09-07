use std::path::PathBuf;

use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::Parse;
use syn::spanned::Spanned;
use syn::{Expr, ExprLit, Ident, Lit, LitStr, MetaNameValue};

use crate::impls::flavor::toml_load::HooqToml;
use crate::impls::flavor::{CheckedHooqToml, LOADED_HOOQ_TOML};

pub fn toml_load(input: TokenStream) -> syn::Result<TokenStream> {
    let span = input.span();
    let flavor_load_method: FlavorLoadMethod = syn::parse2(input)?;

    flavor_load_method
        .register_toml_and_emit_tokens()
        .map_err(|e| syn::Error::new(span, e))
}

enum FlavorLoadMethod {
    File(PathBuf),
    Content(HooqToml),
}

const UNEPXECTED_EXPRESSION_ERROR_MSG: &str = r#"unexpected expression. expected:
- toml_load!()
- toml_load!("(file path or toml content)")
- toml_load!(toml = "(file path or toml content)")
- toml_load!(file = "(file path)")
- toml_load!(path = "(file path)")
- toml_load!(content = "(toml content)")

file path must be absolute or relative to the Cargo.toml.
"#;

impl Parse for FlavorLoadMethod {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let cargo_dir =
            PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string()));

        // 相対パスを Cargo.toml からの絶対パスに変換する
        fn convert_rel2abs(
            path: impl Into<PathBuf>,
            cargo_dir: &std::path::Path,
        ) -> Result<PathBuf, String> {
            let path: PathBuf = path.into();
            if path.is_absolute() {
                return Ok(path);
            }

            let res = cargo_dir.join(path);
            let res_for_debug = res.display().to_string();

            // ここでついでにファイルの存在も確かめられるため、
            // 相対パスが与えられた場合はこの後のファイル存在チェックは冗長ではある
            // 絶対パスの場合を考えそのままとする
            res.canonicalize()
                .map_err(|e| format!("failed to canonicalize path `{res_for_debug}`: {e}"))
        }

        // 空の場合はプロジェクトルート直下かカレントディレクトリの hooq.toml を読む

        if input.is_empty() {
            let dir_path = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
            let path = PathBuf::from(dir_path).join("hooq.toml");
            let path =
                convert_rel2abs(path, &cargo_dir).map_err(|s| syn::Error::new(input.span(), s))?;

            if let Ok(false) | Err(_) = path.try_exists() {
                return Err(syn::Error::new(input.span(), "hooq.toml not found"));
            }

            return Ok(Self::File(path));
        }

        fn toml_or_file_path(
            s: &str,
            cargo_dir: &std::path::Path,
        ) -> Result<FlavorLoadMethod, Option<String>> {
            let toml_err_msg = match toml::from_str(s) {
                Ok(toml) => return Ok(FlavorLoadMethod::Content(toml)),
                Err(e) => e.message().to_string(),
            };

            // 改行を含む場合はPathとして不正なので
            // tomlが崩れたものとみなしエラー内容も返す
            if s.contains('\n') {
                return Err(Some(toml_err_msg));
            }

            // ファイルが存在する場合パスとみる
            // canonicalize 失敗時のみ Some のエラーを返す
            let path = convert_rel2abs(s, cargo_dir).map_err(Some)?;
            if let Ok(true) = path.try_exists() {
                return Ok(FlavorLoadMethod::File(path));
            }

            Err(None)
        }

        // "..." の場合はファイルパス・toml直書きの両方を検討

        if input.peek(LitStr) {
            let lit: LitStr = input.parse()?;
            let s = lit.value();

            return toml_or_file_path(&s, &cargo_dir).map_err(|opt_e| {
                syn::Error::new_spanned(
                    lit,
                    match opt_e {
                        Some(e) => e,
                        None => {
                            "failed to parse as toml or file path (file must exist)".to_string()
                        }
                    },
                )
            });
        }

        // ここからは MetaNameValue しか受け入れないため、
        // peek する

        if !(input.peek(Ident) && input.peek2(syn::Token![=]) && input.peek3(LitStr)) {
            return Err(syn::Error::new(
                input.span(),
                UNEPXECTED_EXPRESSION_ERROR_MSG,
            ));
        }

        // ? を使っているが MetaNameValue へのパースは成功するはず
        match input.parse::<MetaNameValue>()? {
            // toml = "..." の場合はファイルパス・toml直書きの両方を検討
            MetaNameValue {
                path,
                value:
                    Expr::Lit(ExprLit {
                        lit: Lit::Str(lit), ..
                    }),
                ..
            } if path.is_ident("toml") => {
                toml_or_file_path(&lit.value(), &cargo_dir).map_err(|opt_e| {
                    syn::Error::new_spanned(
                        lit,
                        match opt_e {
                            Some(e) => e,
                            None => {
                                "failed to parse as toml or file path (file must exist)".to_string()
                            }
                        },
                    )
                })
            }
            // path = "..." | file = "..." の場合はファイルパスのみ検討
            MetaNameValue {
                path,
                value:
                    Expr::Lit(ExprLit {
                        lit: Lit::Str(lit), ..
                    }),
                ..
            } if path.is_ident("file") || path.is_ident("path") => {
                let path = convert_rel2abs(lit.value(), &cargo_dir)
                    .map_err(|e| syn::Error::new(lit.span(), e))?;
                match path.try_exists() {
                    Ok(true) => Ok(FlavorLoadMethod::File(path)),
                    _ => Err(syn::Error::new_spanned(lit, "file not found")),
                }
            }
            // content = "..." の場合はtoml直書きのみ検討
            MetaNameValue {
                path,
                value:
                    Expr::Lit(ExprLit {
                        lit: Lit::Str(lit), ..
                    }),
                ..
            } if path.is_ident("content") => Ok(Self::Content(
                toml::from_str(&lit.value())
                    .map_err(|e| syn::Error::new_spanned(lit, e.message()))?,
            )),
            _ => Err(syn::Error::new(
                input.span(),
                UNEPXECTED_EXPRESSION_ERROR_MSG,
            )),
        }
    }
}

impl FlavorLoadMethod {
    fn register_toml_and_emit_tokens(self) -> Result<TokenStream, String> {
        match self {
            FlavorLoadMethod::File(path_buf) => inner_for_path_buf(&path_buf),
            FlavorLoadMethod::Content(hooq_toml) => {
                inner_for_content(hooq_toml)?;
                Ok(TokenStream::new())
            }
        }
    }
}

fn inner_for_path_buf(path: &std::path::Path) -> Result<TokenStream, String> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("failed to read file `{}`: {}", path.display(), e))?;

    let hooq_toml: HooqToml = toml::from_str(&content)
        .map_err(|e| format!("failed to parse toml from file `{}`: {}", path.display(), e))?;

    let checked_hooq_toml = CheckedHooqToml::try_from(hooq_toml)?;

    LOADED_HOOQ_TOML.set(checked_hooq_toml);

    let path_display = path.display().to_string();

    Ok(quote! {
        const _: &str = include_str!(#path_display);
    })
}

fn inner_for_content(hooq_toml: HooqToml) -> Result<(), String> {
    let checked_hooq_toml = CheckedHooqToml::try_from(hooq_toml)?;

    LOADED_HOOQ_TOML.set(checked_hooq_toml);

    Ok(())
}
