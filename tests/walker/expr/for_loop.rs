use std::ops::RangeInclusive;

use hooq::hooq;

#[hooq]
fn range(end: usize) -> Result<RangeInclusive<usize>, ()> {
    Ok(0..=end)
}

#[hooq]
fn hoge() -> Result<(), ()> {
    Ok(())
}

#[hooq]
#[hooq::method(.inspect(|_| {
    println!("tag: {:?}", $tag);
}))]
fn func() -> Result<(), ()> {
    for i in range(10)? {
        println!("{i} start");

        if i > 100 {
            return hoge();
        }

        if i > 50 {
            hoge()?;

            return Err(());
        }

        println!("{i} end");
    }

    Ok(())
}

#[test]
fn test() {
    func().unwrap()
}
