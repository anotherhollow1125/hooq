use hooq::hooq;

// ExprGroup と同様の内容

#[hooq]
fn hoge() -> Result<u32, ()> {
    Ok(10)
}

#[hooq]
#[hooq::method(.inspect(|_| {
    println!("tag: {:?}", $tag);
}))]
fn func(flag: bool) -> Result<(), ()> {
    let _ = (2
        * (3 * hoge()? * (hoge()?) * 5 * {
            if !flag {
                return Err(());
            }

            Result::<u32, ()>::Ok({
                let tmp = hoge()?;

                if !flag {
                    return hoge().map(|_| ());
                }

                hoge()? + tmp
            })
        }
        .unwrap()))
        * 6;

    Ok(())
}

#[test]
fn test() {
    func(true).unwrap();
}
