use hooq_macros::hooq;

#[hooq]
fn enresult<T>(t: T) -> Result<T, ()> {
    Ok(t)
}

// TODO: 関数以外にもhooqを付けられるようになったら修正
#[hooq]
#[hooq::method(.inspect(|_| {
    println!("tag: {}, expr: {}", $tag, $expr);
}))]
#[hooq::tag = "(no tag)"]
fn func() -> Result<(), ()> {
    #[allow(unused)]
    mod tmp {
        use util_macros::id;

        use super::enresult;

        // macro_rulesの中身は見ない
        #[allow(unused)]
        macro_rules! tmp {
            () => {
                fn tmp_fn(flag: bool) -> Result<(), ()> {
                    if flag {
                        enresult(())?;

                        return Err(());
                    }

                    Ok(())
                }
            };
        }

        #[hooq::tag = "outer"]
        id! {
            #[allow(unused)]
            fn outer() -> Result<(), ()> {
                enresult(())?;

                Ok(())
            }
        }

        id! {
            #[hooq::tag = "const"]
            const _CONST_VAL: usize = {
                #[hooq::tag = "inner func"]
                fn _f() -> Result<(), ()> {
                    enresult(())?;

                    enresult(())
                }

                10
            };

            #[allow(unused)]
            fn inner() -> Result<(), ()> {
                #[hooq::tag = "inner"]
                enresult(())?;

                Ok(())
            }
        }
    }

    Ok(())
}

#[test]
fn test() {
    func().unwrap();
}
