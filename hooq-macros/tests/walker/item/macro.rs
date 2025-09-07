use hooq_macros::hooq;

#[hooq]
fn enresult<T>(t: T) -> Result<T, ()> {
    Ok(t)
}

#[hooq]
#[hooq::method(.inspect(|_| {
    println!("tag: {}, expr: {}", $tag, $expr_str);
}))]
#[hooq::tag = "(no tag)"]
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
        pub fn outer() -> Result<(), ()> {
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

        pub fn inner() -> Result<(), ()> {
            #[hooq::tag = "inner"]
            enresult(())?;

            Ok(())
        }
    }
}

#[test]
fn test() {
    tmp::outer().unwrap();
    tmp::inner().unwrap();
}
