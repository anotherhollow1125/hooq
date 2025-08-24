use hooq_macros::hooq;

#[hooq]
fn v() -> Result<Vec<usize>, ()> {
    Ok(vec![1, 2, 3])
}

#[hooq]
fn hoge() -> Result<usize, ()> {
    Ok(1)
}

#[hooq]
fn func() -> Result<(), ()> {
    let _ = v()?[hoge()?];

    Ok(())
}

#[test]
fn test() {
    func().unwrap();
}
