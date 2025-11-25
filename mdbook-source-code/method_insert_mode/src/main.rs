use hooq::hooq;

fn failable<T>(val: T) -> Result<T, String> {
    Ok(val)
}

#[hooq]
#[hooq::method(.inspect_err(|_| {
    let _ = "inserted mode";
}))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    failable(())?;

    #[hooq::method(.inspect_err(|_| {
        let _ = "before chainned";
    }).$so_far)]
    failable(())?;

    #[hooq::method(.$so_far.inspect_err(|_| {
        let _ = "after chainned";
    }))]
    failable(())?;

    Ok(())
}
