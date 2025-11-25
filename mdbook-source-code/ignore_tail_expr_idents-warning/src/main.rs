use hooq::hooq;

#[hooq]
#[hooq::method(.inspect_err(|_| {}))]
fn main() {
    let f = || -> Result<(), String> { Err("error!".to_string()) };

    #[hooq::ignore_tail_expr_idents("Err")]
    let g = || -> Result<(), String> { Err("error!".to_string()) };

    #[hooq::tail_expr_idents("!Err")]
    let h = || -> Result<(), String> { Err("error!".to_string()) };

    f().unwrap_err();
    g().unwrap_err();
    h().unwrap_err();
}
