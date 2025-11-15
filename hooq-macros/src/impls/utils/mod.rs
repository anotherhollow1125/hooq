pub use get_attrs::*;
use syn::Path;

pub mod function_info;
mod get_attrs;
pub mod get_hooq_crate_name;
mod source_excerpt;
pub mod source_excerpt_macros;
pub mod unexpected_error_message;

pub fn path_is_end_of(path: &Path, target: &str) -> bool {
    path.segments
        .iter()
        .next_back()
        .is_some_and(|segment| segment.ident == target)
}
