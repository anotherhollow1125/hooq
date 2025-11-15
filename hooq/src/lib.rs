#![doc = include_str!("../docs/README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/anotherhollow1125/hooq/refs/heads/main/assets/hooq_logo.svg"
)]
#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/anotherhollow1125/hooq/refs/heads/main/assets/hooq_logo.svg"
)]

pub use hooq_helpers::{BindingPayload, HooqMeta};
pub use hooq_macros::hooq;

pub mod source_excerpt_helpers;
