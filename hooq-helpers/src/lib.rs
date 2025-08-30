#[derive(Debug)]
pub struct HooqMeta {
    pub line: usize,
    pub column: usize,
    pub path: &'static str,
    pub abs_path: &'static str,
    pub file: &'static str,
    pub expr: &'static str,
    pub count: &'static str,
}
