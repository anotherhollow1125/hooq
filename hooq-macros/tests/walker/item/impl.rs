use hooq_macros::hooq;

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
mod tmp {
    use util_macros::id;

    use super::hoge;

    pub struct S;

    #[hooq::tag = "impl"]
    impl S {
        #[hooq::tag = "const"]
        const _CONST_VAL: usize = {
            #[hooq::tag = "inner func"]
            fn _f() -> Result<(), ()> {
                hoge()?;

                hoge()
            }

            10
        };

        #[hooq::tag = "impl related function"]
        pub fn g() -> Result<(), ()> {
            hoge()?;

            Ok(())
        }

        #[hooq::tag = "impl related function 2 (not Result)"]
        pub fn h() -> bool {
            true
        }

        #[allow(unused)]
        #[hooq::tag = "impl method"]
        fn i(&self) -> Result<(), ()> {
            let res = Ok(());

            hoge()?;

            res
        }

        #[hooq::tag = "impl method 2 (not Result)"]
        #[allow(unused)]
        fn j(&self) -> bool {
            true
        }

        #[hooq::tag = "outer"]
        id! {
            pub fn outer() -> Result<(), ()> {
                hoge()?;

                Ok(())
            }
        }

        id! {
            pub fn inner() -> Result<(), ()> {
                #[hooq::tag = "inner"]
                hoge()?;

                Ok(())
            }
        }
    }
}

#[test]
fn test() {
    tmp::S::g().unwrap();
    let _ = tmp::S::h();
    tmp::S::outer().unwrap();
    tmp::S::inner().unwrap();
}
