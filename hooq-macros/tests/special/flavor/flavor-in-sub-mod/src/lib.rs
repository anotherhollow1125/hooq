pub mod sub;

#[test]
fn test() {
    use sub::{Checker, func_err, func_ok};

    let mut checker = Checker::new();
    func_ok(&mut checker).unwrap();
    assert!(checker.checked());

    let mut checker = Checker::new();
    func_err(&mut checker).unwrap_err();
    assert!(checker.checked());
}
