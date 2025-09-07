use std::collections::HashMap;

use crate::impls::flavor::Flavor;

mod default;
mod empty;
mod hook;

// NOTE:
// default や empty までファイル分けする必要性は薄いが、
// preset flavor の一つであることを強調するために用意した
pub fn preset_flavors() -> HashMap<String, Flavor> {
    [
        ("default", default::default_flavor()),
        ("empty", empty::empty_flavor()),
        ("hook", hook::hook_flavor()),
    ]
    .into_iter()
    .map(|(s, f)| (s.to_string(), f))
    .collect()
}
