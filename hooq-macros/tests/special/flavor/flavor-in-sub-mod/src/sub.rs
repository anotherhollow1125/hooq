use hooq::hooq;

pub struct Checker(bool);

impl Checker {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self(false)
    }

    fn call(&mut self) {
        self.0 = true;
    }

    pub fn checked(&self) -> bool {
        self.0
    }
}

#[hooq(flavor = "my-flavor-sub-mod")]
#[hooq::checker = checker]
#[allow(clippy::result_unit_err)]
pub fn func_ok(checker: &mut Checker) -> Result<(), ()> {
    Ok(())
}

#[allow(clippy::result_unit_err)]
#[hooq(flavor = "my-flavor-sub-mod.err")]
#[hooq::checker = checker]
pub fn func_err(checker: &mut Checker) -> Result<(), ()> {
    Err(())
}
