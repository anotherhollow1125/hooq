use hooq::hooq;

struct Strct;

impl Strct {
    #[hooq]
    fn method(&self, _val1: usize, _val2: usize) -> Result<(), ()> {
        Ok(())
    }
}

#[hooq]
fn get_strct() -> Result<Strct, ()> {
    Ok(Strct)
}

#[hooq]
fn get_val() -> Result<usize, ()> {
    Ok(10)
}

#[hooq]
#[hooq::method(.inspect(|_| {
    println!("tag: {:?}", $tag);
}))]
fn func() -> Result<(), ()> {
    get_strct()?.method(get_val()?, get_val()?)?;

    Ok(())
}

#[test]
fn test() {
    func().unwrap();
}
