use hooq::{hooq, toml_load};

toml_load!(file = r#""my flavors".toml"#);

struct Checker(bool);

impl Checker {
    #[allow(unused)]
    fn new() -> Self {
        Self(false)
    }

    fn call(&mut self) {
        self.0 = true;
    }

    #[allow(unused)]
    fn checked(&self) -> bool {
        self.0
    }
}

#[hooq(flavor = "my-flavor-file")]
#[hooq::checker = checker]
#[allow(unused)]
fn func_ok(checker: &mut Checker) -> Result<(), ()> {
    Ok(())
}

#[allow(unused)]
#[hooq(flavor = "my-flavor-file.err")]
#[hooq::checker = checker]
fn func_err(checker: &mut Checker) -> Result<(), ()> {
    Err(())
}

#[test]
fn test() {
    let mut checker = Checker::new();
    func_ok(&mut checker).unwrap();
    assert!(checker.checked());

    let mut checker = Checker::new();
    func_err(&mut checker).unwrap_err();
    assert!(checker.checked());
}
