use std::any::Any;
use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;

pub struct BindingPayload {
    pub expr: String,
    pub value: Rc<dyn Any>,
}

impl Debug for BindingPayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Binding").field("expr", &self.expr).finish()
    }
}

#[derive(Debug)]
pub struct HooqMeta {
    pub line: usize,
    pub column: usize,
    pub path: &'static str,
    pub abs_path: &'static str,
    pub file: &'static str,
    pub expr: &'static str,
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
