use hooq_macros::hooq;

#[hooq]
fn hoge() -> Result<usize, ()> {
    Ok(10)
}

#[hooq]
#[hooq::method(.inspect(|_| {
    println!("tag: {}", $tag);
}))]
#[hooq::tag = "(no tag)"]
#[hooq::tail_expr_idents("Ok", "Err")]
fn func(flag: bool) -> Result<(), ()> {
    match hoge()? {
        0..=10 => {
            println!("Matched 0..=10");
            hoge()?;

            if !flag {
                return hoge().map(|_| ());
            }

            Ok(())
        }
        _ => {
            println!("Matched other case");

            if !flag {
                return Ok(());
            }

            Err(())
        }
    }
}

#[test]
fn test() {
    func(true).unwrap();
}
