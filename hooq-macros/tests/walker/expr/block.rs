use hooq_macros::hooq;

#[hooq]
fn hoge() -> Result<(), ()> {
    Ok(())
}

#[hooq]
#[hooq::method(.inspect(|_| {
    println!("tag: {}", $tag);
}))]
#[hooq::tail_expr_idents("Ok", "Err")]
#[allow(clippy::unit_arg)]
fn func(flag: bool) -> Result<(), ()> {
    #[hooq::tag = "1"]
    {
        println!("beep");

        #[hooq::tag = "2"]
        Ok({
            #[hooq::tag = "3"]
            Ok({
                #[hooq::tag = "4"]
                {
                    if flag {
                        return hoge();
                    }

                    #[hooq::tag = "5"]
                    {
                        if flag {
                            return Err(());
                        }

                        Ok(())
                    }?;

                    #[hooq::tag = "6"]
                    {
                        println!("beepbeep");
                    }
                }
            })
        }?)
    }
}

#[test]
fn test() {
    func(true).unwrap();
}
