use hooq_macros::hooq;

#[hooq]
fn enresult<T>(t: T) -> Result<T, ()> {
    Ok(t)
}

#[hooq]
#[hooq::method(.inspect(|_| {
    println!("tag: {}, expr: {}", $tag, stringify!($source));
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

                Err(())
            }
        };
    }

    #[hooq::tag = "outer"]
    id! {
        pub fn outer() -> Result<(), ()> {
            enresult(())?;

            Err(())
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

            Err(())
        }
    }
}

#[test]
fn test() {
    tmp::outer().unwrap_err();
    tmp::inner().unwrap_err();
}
