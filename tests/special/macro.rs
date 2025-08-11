use hooq::hooq;
use util_macros::empty;

#[hooq]
fn enresult<T>(t: T) -> Result<T, ()> {
    Ok(t)
}

// 最初の内容は他のテストと重複している
// 検証していることを確実に確かめるべく特別に切り出した

#[hooq]
#[hooq::method(.inspect(|_| {
    println!("tag: {}, expr: {}", $tag, $expr);
}))]
fn func() -> Result<(), ()> {
    // Stmt としてパースされるマクロ
    #[hooq::tag("outer")]
    println!("{}", enresult(10)?);

    println!(
        "{}",
        #[hooq::tag("inner")]
        enresult(20)?
    );

    // Item としてパースされるマクロ
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

        #[hooq::tag("outer")]
        id! {
            #[allow(unused)]
            fn outer() -> Result<(), ()> {
                enresult(())?;

                Ok(())
            }
        }

        id! {
            #[hooq::tag("const")]
            const _CONST_VAL: usize = {
                #[hooq::tag("inner func")]
                fn _f() -> Result<(), ()> {
                    enresult(())?;

                    enresult(())
                }

                10
            };

            #[allow(unused)]
            fn inner() -> Result<(), ()> {
                #[hooq::tag("inner")]
                enresult(())?;

                Ok(())
            }
        }
    }

    // Expr としてパースされるマクロ
    let _ = #[hooq::tag("outer")]
    vec![enresult(10)?, 20, enresult(30)?];

    let _ = vec![
        #[hooq::tag("inner 1")]
        enresult(10)?,
        #[hooq::tag("inner 2")]
        enresult(20)?,
        #[hooq::tag("inner 3")]
        enresult(30)?,
    ];

    // Rustコードとして解釈できないものについては無理にパースしない
    // empty! と serde_json! でこのことを検証

    empty! {
        Hey! This string is not rust code but an English sentence! How do you feel?
    }

    let _ = serde_json::json!({
        "key": "value",
        "array": enresult([1, 2, 3])?,
    });

    Ok(())
}

#[test]
fn test() {
    func().unwrap();
}
