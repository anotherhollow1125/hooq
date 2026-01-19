use std::sync::OnceLock;

use proc_macro_crate::{FoundCrate, crate_name};
use syn::Path;

pub fn get_hooq_crate_name() -> &'static str {
    static RES: OnceLock<String> = OnceLock::new();

    RES.get_or_init(|| {
        let found_crate = crate_name("hooq").or_else(|_| crate_name("hooq-macros"));

        match found_crate {
            Ok(FoundCrate::Itself) => "hooq".to_string(), // ファサードのREADMEにてこうするしかなかったため
            Ok(FoundCrate::Name(name)) => name.replace("-", "_").to_string(),
            Err(_) => "hooq".to_string(),
        }
    })
}

pub fn get_source_excerpt_helpers_name_space() -> Path {
    let crate_name = get_hooq_crate_name();

    let source_excerpt_helpers_name_space = match crate_name {
        "hooq_macros" => "",
        _ => "::source_excerpt_helpers",
    };

    syn::parse_str::<Path>(&format!(
        "::{}{}",
        crate_name, source_excerpt_helpers_name_space
    ))
    .unwrap()
}
