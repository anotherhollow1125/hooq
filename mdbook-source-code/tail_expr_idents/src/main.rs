use hooq::hooq;

fn fallible<T>(val: T) -> Result<T, String> {
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

        fallible(())
    };

    #[hooq::tail_expr_idents("Err", "fallible")]
    let _: Result<(), String> = {
        let _: Result<(), String> = {
            let res = "error".to_string();
            Err(res)
        };

        fallible(()) // This will be hooked because of tail_expr_idents.
    };

    Ok(())
}
