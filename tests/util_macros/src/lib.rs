use proc_macro::TokenStream as S;

#[proc_macro]
pub fn id(t: S) -> S {
    t
}

#[proc_macro]
pub fn empty(_: S) -> S {
    S::new()
}
