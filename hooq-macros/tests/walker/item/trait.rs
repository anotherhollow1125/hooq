use hooq_macros::hooq;

// 確認内容は impl.rs と同じ

#[hooq]
#[hooq::method(.inspect(|_| {
    println!("tag: {}", $tag);
}))]
#[hooq::tag = "(no tag)"]
mod tmp {
    use util_macros::id;

    #[allow(unused)]
    fn hoge() -> Result<(), ()> {
        Err(())
    }

    pub trait Trit {
        #[hooq::tag = "const"]
        const _CONST_VAL: usize = {
            #[hooq::tag = "inner func"]
            fn _f() -> Result<(), ()> {
                hoge()?;

                hoge()
            }

            10
        };

        #[hooq::tag = "related function"]
        fn g() -> Result<(), ()> {
            hoge()?;

            Err(())
        }

        #[hooq::tag = "related function 2 (not Result)"]
        fn h() -> bool {
            true
        }

        #[allow(unused)]
        #[hooq::tag = "method"]
        fn i(&self) -> Result<(), ()> {
            let res = Err(());

            hoge()?;

            res
        }

        #[hooq::tag = "method 2 (not Result)"]
        #[allow(unused)]
        fn j(&self) -> bool {
            true
        }

        #[hooq::tag = "outer"]
        id! {
            #[allow(unused)]
            fn outer() -> Result<(), ()> {
                hoge()?;

                Err(())
            }
        }

        id! {
            #[allow(unused)]
            fn inner() -> Result<(), ()> {
                #[hooq::tag = "inner"]
                hoge()?;

                Err(())
            }
        }
    }
}

#[test]
fn test() {
    struct T;

    impl tmp::Trit for T {}

    <T as tmp::Trit>::g().unwrap_err();
    let _ = <T as tmp::Trit>::h();
}
