#![doc = include_str!("../docs/README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/anotherhollow1125/hooq/refs/heads/main/assets/hooq_logo.svg"
)]
#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/anotherhollow1125/hooq/refs/heads/main/assets/hooq_logo.svg"
)]

use std::any::Any;
use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;

/// A binding payload that contains the expression string and its value.
///
/// Value of this struct is stored in the `bindings` map's value of `HooqMeta` struct.
pub struct BindingPayload {
    pub expr: String,
    pub value: Rc<dyn Any>,
}

impl Debug for BindingPayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Binding").field("expr", &self.expr).finish()
    }
}

/// Metadata about the invocation of the `hooq` macro.
///
/// You can use access this metadata via $hook_meta variable inside the macro body.
/// You also can use this metadata via `hook` flavor method.
#[derive(Debug)]
pub struct HooqMeta {
    pub line: usize,
    pub column: usize,
    pub path: &'static str,
    pub file: &'static str,
    pub expr_str: &'static str,
    pub count: &'static str,
    pub bindings: HashMap<String, BindingPayload>,
}

impl HooqMeta {
    pub fn get_binding<T>(&self, key: &str) -> Option<Rc<T>>
    where
        T: 'static,
    {
        self.bindings
            .get(key)
            .and_then(|binding| Rc::clone(&binding.value).downcast().ok())
    }
}
