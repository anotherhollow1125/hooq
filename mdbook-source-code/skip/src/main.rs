use hooq::hooq;

#[hooq]
#[hooq::method(.inspect_err(|_| {}))]
fn func1() -> Result<(), String> {
    match fallible(fallible(()))? {
        Ok(()) => Ok(()),
        Err(s) => Err(s),
    }
}

#[hooq]
#[hooq::method(.inspect_err(|_| {}))]
fn func2() -> Result<(), String> {
    #[hooq::skip]
    match fallible(fallible(()))? {
        Ok(()) => Ok(()),
        Err(s) => Err(s),
    } // Not hooked here.
}

fn fallible<T>(val: T) -> Result<T, String> {
    Ok(val)
}

fn main() {
    let _ = func1();
    let _ = func2();
}
