use hooq::hooq;

fn fallible<T>(val: T) -> Result<T, String> {
    Ok(val)
}

#[hooq]
#[hooq::method(.inspect_err(|_| {
    let _ = "inserted mode";
}))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    fallible(())?;

    #[hooq::method(.inspect_err(|_| {
        let _ = "before chained";
    }).$so_far)]
    fallible(())?;

    #[hooq::method(.$so_far.inspect_err(|_| {
        let _ = "after chained";
    }))]
    fallible(())?;

    Ok(())
}
