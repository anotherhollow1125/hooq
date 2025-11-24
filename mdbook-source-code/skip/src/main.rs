use hooq::hooq;

#[hooq]
#[hooq::method(.inspect_err(|_| {}))]
fn main() -> Result<(), String> {
    #[hooq::skip]
    match failable(failable(failable(()))) {
        Ok(v) => match v {
            Ok(v) =>
            {
                #[hooq::skip]
                match v {
                    Ok(v) => Ok(v),
                    Err(s) => Err(s),
                }
            } // Not hooked here
            Err(s) => Err(s),
        },
        Err(s) => Err(s),
    } // Not hooked here
}

fn failable<T>(val: T) -> Result<T, String> {
    Ok(val)
}
