use std::collections::HashMap;

use serde::Deserialize;

pub mod apply;

#[derive(Deserialize, Debug, Clone)]
pub struct HooqToml {
    #[serde(flatten)]
    pub flavors: HashMap<String, FlavorTable>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct FlavorTable {
    pub trait_uses: Option<Vec<String>>,
    pub method: Option<String>,
    pub hook_targets: Option<Vec<String>>,
    pub tail_expr_idents: Option<Vec<String>>,
    pub ignore_tail_expr_idents: Option<Vec<String>>,
    pub result_types: Option<Vec<String>>,
    pub hook_in_macros: Option<bool>,
    pub bindings: Option<HashMap<String, String>>,
    #[serde(flatten)]
    pub sub_flavors: HashMap<String, FlavorTable>,
}

#[cfg(test)]
mod tests {
    use super::HooqToml;

    #[test]
    fn test_toml_load() {
        let hooq_toml = r#"[default]
method = """.inspect_err(|_| {
    if $flag {
        ::std::eprintln!("[{}:{}] error occurred.", $file, $line);
    }
})"""
hook_targets = ["?"]
tail_expr_idents = ["Err"]
ignore_tail_expr_idents = ["Ok"]
hook_in_macros = true
bindings = { flag = "false" }

[my_flavor]
trait_uses = ["my_crate::MyTrait"]
method = """.my_hook_method(|| $hooq_meta)"""
hook_targets = ["?", "return", "tail_expr"]
tail_expr_idents = ["Ok", "Err"]
result_types = ["Result", "MyResult"]
hook_in_macros = true
bindings = {}

[my_flavor.release]
method = """.my_hook_method(|| $hooq_meta)"""
hook_targets = ["?"]
tail_expr_idents = []
result_types = ["Result", "MyResult"]
hook_in_macros = true
bindings = {}
"#;

        let hooq_toml: HooqToml = toml::from_str(hooq_toml).unwrap();
        println!("{hooq_toml:#?}");
    }

    #[test]
    fn test_empty_toml_load() {
        let hooq_toml = "";

        let hooq_toml: HooqToml = toml::from_str(hooq_toml).unwrap();
        println!("{hooq_toml:#?}");
    }
}
