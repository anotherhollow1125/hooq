use hooq_macros::hooq;

// static.rs とほぼ中身は同じ

#[hooq]
#[hooq::method(.inspect(|_| {
    println!("tag: {}", $tag);
}))]
#[hooq::tag = "(no tag)"]
#[hooq::tail_expr_idents("Ok", "Err")]
mod tmp {
    #[allow(unused)]
    fn hoge() -> Result<(), ()> {
        Err(())
    }

    pub const STATIC_VAR: u32 = {
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

                Err(())
            };

            res
        }

        10
    };
}

#[test]
fn test() {
    let _ = tmp::STATIC_VAR;
}
