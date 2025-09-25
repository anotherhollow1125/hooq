use hooq_macros::hooq;

#[hooq]
fn func(_: ()) -> Result<(), ()> {
    Err(())
}

#[hooq]
#[hooq::method(.inspect(|_| {
    let path = $path;
    let line = $line;
    let expr = $expr_str;

    ::std::eprintln!("nested @
path: {path},
line: {line}
expr: {expr}");
}))]
fn nested() -> Result<(), ()> {
    func(func(func(func(())?)?)?)?;
    Err(())
}

#[test]
fn test() {
    nested().unwrap_err();
}
