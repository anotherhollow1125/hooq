use std::collections::HashMap;

use crate::impls::flavor::Flavor;

#[cfg(feature = "anyhow")]
mod anyhow;
mod default;
mod empty;
#[cfg(feature = "eyre")]
mod eyre;
mod hook;
#[cfg(feature = "log")]
mod log;
#[cfg(feature = "tracing")]
mod tracing;

// NOTE:
// default や empty までファイル分けする必要性は薄いが、
// preset flavor の一つであることを強調するために用意した
pub fn preset_flavors() -> HashMap<String, Flavor> {
    [
        ("default", default::default_flavor()),
        ("empty", empty::empty_flavor()),
        ("hook", hook::hook_flavor()),
        #[cfg(feature = "log")]
        ("log", log::log_flavor()),
        #[cfg(feature = "anyhow")]
        ("anyhow", anyhow::anyhow_flavor()),
        #[cfg(feature = "eyre")]
        ("eyre", eyre::eyre_flavor()),
        #[cfg(feature = "tracing")]
        ("tracing", tracing::tracing_flavor()),
    ]
    .into_iter()
    .map(|(s, f)| (s.to_string(), f))
    .collect()
}
