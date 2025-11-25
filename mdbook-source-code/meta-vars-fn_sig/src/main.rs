use hooq::hooq;

fn failable<T>(val: T) -> Result<T, String> {
    Ok(val)
}

#[hooq]
#[hooq::method(.inspect_err(|_| {
    let _fn_sig = $fn_sig;
}))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    failable(())?;

    (|| -> Result<(), String> {
        failable(())?;

        Ok(())
    })()?;

    Ok(())
}
