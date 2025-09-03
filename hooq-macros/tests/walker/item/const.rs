use hooq_macros::hooq;

// static.rs とほぼ中身は同じ

#[hooq]
#[allow(unused)]
fn hoge() -> Result<(), ()> {
    Ok(())
}

#[hooq]
#[hooq::method(.inspect(|_| {
    println!("tag: {}", $tag);
}))]
#[hooq::tag = "(no tag)"]
#[hooq::tail_expr_idents("Ok", "Err")]
fn func() -> Result<(), ()> {
    const _STATIC_VAR: u32 = {
        #[hooq::tag = "inner func"]
        fn _f(flag: bool) -> Result<(), ()> {
            if flag {
                return Err(());
            }

            hoge()?;

            #[hooq::tag = "deep"]
            let res = {
                if flag {
                    return hoge();
                }

                Ok(())
            };

            res
        }

        10
    };

    Ok(())
}

#[test]
fn test() {
    func().unwrap();
}
