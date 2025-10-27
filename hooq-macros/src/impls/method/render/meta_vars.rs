use std::fmt::Display;
use std::str::FromStr;

pub enum MetaVars {
    Line,
    Column,
    Path,
    AbsPath,
    File,
    Expr,
    ExprStr,
    ExprStrShort,
    Count,
    FnName,
    FnSig,
    Bindings,
    HooqMeta,
}

pub const META_VARS_LIST: [MetaVars; 13] = [
    MetaVars::Line,
    MetaVars::Column,
    MetaVars::Path,
    MetaVars::AbsPath,
    MetaVars::File,
    MetaVars::Expr,
    MetaVars::ExprStr,
    MetaVars::ExprStrShort,
    MetaVars::Count,
    MetaVars::FnName,
    MetaVars::FnSig,
    MetaVars::Bindings,
    MetaVars::HooqMeta,
];

impl FromStr for MetaVars {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "line" => Ok(MetaVars::Line),
            "column" | "col" => Ok(MetaVars::Column),
            "path" => Ok(MetaVars::Path),
            "abspath" | "abs_path" => Ok(MetaVars::AbsPath),
            "file" => Ok(MetaVars::File),
            "expr" => Ok(MetaVars::Expr),
            "expr_str" => Ok(MetaVars::ExprStr),
            "expr_str_short" => Ok(MetaVars::ExprStrShort),
            "nth" | "count" => Ok(MetaVars::Count),
            "fnname" | "fn_name" => Ok(MetaVars::FnName),
            "fnsig" | "fn_sig" => Ok(MetaVars::FnSig),
            "bindings" | "vars" => Ok(MetaVars::Bindings),
            "hooqmeta" | "hooq_meta" => Ok(MetaVars::HooqMeta),
            binding => Err(binding.to_string()),
        }
    }
}

impl Display for MetaVars {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                MetaVars::Line => "line",
                MetaVars::Column => "column",
                MetaVars::Path => "path",
                MetaVars::AbsPath => "abs_path",
                MetaVars::File => "file",
                MetaVars::Expr => "expr",
                MetaVars::ExprStr => "expr_str",
                MetaVars::ExprStrShort => "expr_str_short",
                MetaVars::Count => "count",
                MetaVars::FnName => "fn_name",
                MetaVars::FnSig => "fn_sig",
                MetaVars::Bindings => "bindings",
                MetaVars::HooqMeta => "hooq_meta",
            }
        )
    }
}
