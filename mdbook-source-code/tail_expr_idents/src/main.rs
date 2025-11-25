use hooq::hooq;

fn failable<T>(val: T) -> Result<T, String> {
    Ok(val)
}

#[hooq]
#[hooq::method(.inspect_err(|_| {}))]
fn main() -> Result<(), String> {
    let _: Result<(), String> = {
        let _: Result<(), String> = {
            let res = "error".to_string();
            Err(res)
        };

        failable(())
    };

    #[hooq::tail_expr_idents("Err", "failable")]
    let _: Result<(), String> = {
        let _: Result<(), String> = {
            let res = "error".to_string();
            Err(res)
        };

        failable(()) // This will be hooked because of tail_expr_idents.
    };

    Ok(())
}
