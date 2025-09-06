use std::collections::HashMap;

use crate::impls::flavor::Flavor;

mod default;

pub fn preset_flavors() -> HashMap<String, Flavor> {
    [("default", default::default_flavor())]
        .into_iter()
        .map(|(s, f)| (s.to_string(), f))
        .collect()
}
