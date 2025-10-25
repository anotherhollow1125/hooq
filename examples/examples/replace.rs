use hooq::hooq;

#[hooq]
fn enresult<T>(val: T) -> Result<T, ()> {
    Ok(val)
}

#[hooq(custom3)]
fn func() -> Result<(), ()> {
    if enresult(false)? {
        return Err::<(), ()>(());
    }

    enresult(())?;

    Ok(())
}

fn main() {
    func().unwrap();
}
