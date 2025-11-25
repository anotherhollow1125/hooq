use hooq::hooq;

fn failable<T>(val: T) -> Result<T, String> {
    Ok(val)
}

#[hooq]
#[hooq::method(.inspect_err(|_| {}))]
fn main() {
    let f = || -> Result<(), String> { failable(()) };

    #[hooq::ignore_tail_expr_idents("failable")]
    let g = || -> Result<(), String> { failable(()) };

    #[hooq::tail_expr_idents("!failable")]
    let h = || -> Result<(), String> { failable(()) };

    f().unwrap();
    g().unwrap();
    h().unwrap();
}
