use std::collections::HashMap;

use crate::impls::flavor::{FlavorNode, FlavorSettings};

pub fn default_flavor() -> FlavorNode {
    FlavorNode {
        settings: FlavorSettings::default(),
        sub_flavors: HashMap::new(),
    }
}
