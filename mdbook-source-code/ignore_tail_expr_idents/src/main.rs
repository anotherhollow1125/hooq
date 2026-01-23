use hooq::hooq;

fn fallible<T>(val: T) -> Result<T, String> {
    Ok(val)
}

#[hooq]
#[hooq::method(.inspect_err(|_| {}))]
fn main() {
    let f = || -> Result<(), String> { fallible(()) };

    #[hooq::ignore_tail_expr_idents("fallible")]
    let g = || -> Result<(), String> { fallible(()) };

    #[hooq::tail_expr_idents("!fallible")]
    let h = || -> Result<(), String> { fallible(()) };

    f().unwrap();
    g().unwrap();
    h().unwrap();
}
