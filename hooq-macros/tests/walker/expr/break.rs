use hooq_macros::hooq;

#[hooq]
fn hoge() -> Result<(), ()> {
    Err(())
}

#[hooq]
#[hooq::method(.inspect(|_| {
    println!("tag: {}", $tag);
}))]
#[hooq::tag = "(no tag)"]
#[hooq::tail_expr_idents("Ok", "Err")]
fn func(flag: bool) -> Result<(), ()> {
    let mut counter = 0;

    loop {
        counter += 1;

        if counter > 2 {
            break;
        }
    }

    loop {
        counter += 1;

        if counter > 3 {
            break Err(());
        }
    }?;

    #[hooq::tag = "label"]
    'outer: loop {
        counter += 1;

        if counter > 4 {
            loop {
                counter += 1;

                if counter > 5 {
                    break 'outer;
                }
            }
        }
    }

    #[hooq::tag = "check nest"]
    loop {
        counter += 1;

        if counter > 10 {
            break {
                #[hooq::tag = "nest"]
                {
                    hoge()?;

                    #[hooq::tag = "return"]
                    if !flag {
                        return Err(());
                    }

                    Err(())
                }
            };
        }
    }
}

#[test]
fn test() {
    func(true).unwrap_err();
}
