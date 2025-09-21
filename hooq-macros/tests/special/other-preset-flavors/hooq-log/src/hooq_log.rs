use hooq::hooq;

#[hooq(log)]
pub fn a() -> Result<(), &'static str> {
    Err("error!")
}

#[hooq(log)]
pub fn b() -> Result<(), &'static str> {
    a()?;

    Err("err")
}

#[hooq(log)]
pub fn c() -> Result<(), &'static str> {
    b()?;

    Err("err")
}
