use std::fmt::Display;
use std::str::FromStr;

#[derive(Eq, PartialEq, Clone, Copy)]
pub enum MetaVars {
    Line,
    Column,
    Path,
    File,
    Expr,
    Source,
    Count,
    FnName,
    FnSig,
    SoFar,
    Bindings,
    HooqMeta,
}

pub const META_VARS_LIST: [MetaVars; 12] = [
    MetaVars::Line,
    MetaVars::Column,
    MetaVars::Path,
    MetaVars::File,
    MetaVars::Expr,
    MetaVars::Source,
    MetaVars::Count,
    MetaVars::FnName,
    MetaVars::FnSig,
    MetaVars::SoFar,
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
            "file" => Ok(MetaVars::File),
            "expr" => Ok(MetaVars::Expr),
            "source" => Ok(MetaVars::Source),
            "nth" | "count" => Ok(MetaVars::Count),
            "fnname" | "fn_name" => Ok(MetaVars::FnName),
            "fnsig" | "fn_sig" => Ok(MetaVars::FnSig),
            "sofar" | "so_far" => Ok(MetaVars::SoFar),
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
                MetaVars::File => "file",
                MetaVars::Expr => "expr",
                MetaVars::Source => "source",
                MetaVars::Count => "count",
                MetaVars::FnName => "fn_name",
                MetaVars::FnSig => "fn_sig",
                MetaVars::SoFar => "so_far",
                MetaVars::Bindings => "bindings",
                MetaVars::HooqMeta => "hooq_meta",
            }
        )
    }
}
