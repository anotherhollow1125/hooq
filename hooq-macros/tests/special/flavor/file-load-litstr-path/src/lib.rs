use hooq::{hooq, toml_load};

// file-load-litstr-content にあたるテストは custom

toml_load!("./src/tomls/hooq.toml");

// ↑テストとして残しにくいので残していないが、絶対パスの場合も検証済み
// ファイルが存在する場合: 問題なし
// ファイルが存在しない場合: failed to parse as toml or file path (file must exist)

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

#[hooq(flavor = "my-flavor-litstr")]
#[hooq::checker = checker]
#[allow(unused)]
fn func_ok(checker: &mut Checker) -> Result<(), ()> {
    Ok(())
}

#[allow(unused)]
#[hooq(flavor = "my-flavor-litstr.err")]
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
