use hooq::hooq;

#[hooq(log)]
pub fn a() -> Result<(), &'static str> {
    Err("error!")
}

#[hooq(log::error)]
pub fn b() -> Result<(), &'static str> {
    a()?;

    Err("err")
}

#[hooq(log::warn)]
pub fn c() -> Result<(), &'static str> {
    b()?;

    Err("err")
}

#[hooq(log::info)]
pub fn d() -> Result<(), &'static str> {
    c()?;

    Err("err")
}

#[hooq(log::debug)]
pub fn e() -> Result<(), &'static str> {
    d()?;

    Err("err")
}

#[hooq(log::trace)]
pub fn f() -> Result<(), &'static str> {
    e()?;

    Err("err")
}
