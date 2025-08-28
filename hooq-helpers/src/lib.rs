#[derive(Debug)]
pub struct HooqInfo {
    pub line: usize,
    pub column: usize,
    pub path: &'static str,
    pub abs_path: &'static str,
    pub file: &'static str,
    pub expr: &'static str,
    pub count: &'static str,
}
