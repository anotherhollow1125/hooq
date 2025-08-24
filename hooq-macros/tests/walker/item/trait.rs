use hooq_macros::hooq;
use util_macros::id;

// 確認内容は impl.rs と同じ

#[hooq]
#[allow(unused)]
fn hoge() -> Result<(), ()> {
    Ok(())
}

#[hooq]
#[hooq::method(.inspect(|_| {
    println!("tag: {}", $tag);
}))]
fn func() -> Result<(), ()> {
    trait Trit {
        #[hooq::tag("const")]
        const _CONST_VAL: usize = {
            #[hooq::tag("inner func")]
            fn _f() -> Result<(), ()> {
                hoge()?;

                hoge()
            }

            10
        };

        #[allow(unused)]
        #[hooq::tag("related function")]
        fn g() -> Result<(), ()> {
            hoge()?;

            Ok(())
        }

        #[hooq::tag("related function 2 (not Result)")]
        #[allow(unused)]
        fn h() -> bool {
            true
        }

        #[allow(unused)]
        #[hooq::tag("method")]
        fn i(&self) -> Result<(), ()> {
            let res = Ok(());

            hoge()?;

            res
        }

        #[hooq::tag("method 2 (not Result)")]
        #[allow(unused)]
        fn j(&self) -> bool {
            true
        }

        #[hooq::tag("outer")]
        id! {
            #[allow(unused)]
            fn outer() -> Result<(), ()> {
                hoge()?;

                Ok(())
            }
        }

        id! {
            #[allow(unused)]
            fn inner() -> Result<(), ()> {
                #[hooq::tag("inner")]
                hoge()?;

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
