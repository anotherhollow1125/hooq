use hooq::hooq;

fn fallible<T>(val: T) -> Result<T, String> {
    Ok(val)
}

fn wrapper<T, E>(r: Result<T, E>) -> Result<T, E>
where
    E: std::fmt::Debug,
{
    if let Err(e) = &r {
        println!("Error occurred: {:?}", e);
    }

    r
}

#[hooq]
#[hooq::method(wrapper($expr))]
fn main() -> Result<(), String> {
    fallible(())?;

    Ok(())
}
