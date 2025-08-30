use std::fmt::Display;
use std::str::FromStr;

pub enum MetaVars {
    Line,
    Column,
    Path,
    AbsPath,
    File,
    Expr,
    Count,
    FnName,
    FnSig,
    HooqMeta,
}

pub const META_VARS_LIST: [MetaVars; 10] = [
    MetaVars::Line,
    MetaVars::Column,
    MetaVars::Path,
    MetaVars::AbsPath,
    MetaVars::File,
    MetaVars::Expr,
    MetaVars::Count,
    MetaVars::FnName,
    MetaVars::FnSig,
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
            "nth" | "count" => Ok(MetaVars::Count),
            "fnname" | "fn_name" => Ok(MetaVars::FnName),
            "fnsig" | "fn_sig" => Ok(MetaVars::FnSig),
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
                MetaVars::Count => "count",
                MetaVars::FnName => "fn_name",
                MetaVars::FnSig => "fn_sig",
                MetaVars::HooqMeta => "hooq_meta",
            }
        )
    }
}
