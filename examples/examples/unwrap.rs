use hooq::hooq;

fn enresult<T>(val: T) -> Result<T, String> {
    Ok(val)
}

#[hooq]
#[hooq::method(.unwrap()!)]
pub fn func_unwrap() {
    enresult(())?;
}

fn main() {
    func_unwrap();
}
