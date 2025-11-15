use hooq_macros::hooq;
use util_macros::empty;

#[hooq]
fn enresult<T>(t: T) -> Result<T, ()> {
    Ok(t)
}

// 最初の内容は他のテストと重複している
// 検証していることを確実に確かめるべく特別に切り出した

#[hooq]
#[hooq::method(.inspect(|_| {
    println!("tag: {}, expr: {}", $tag, stringify!($source));
}))]
#[hooq::tag = "(no tag)"]
fn func() -> Result<(), ()> {
    // Stmt としてパースされるマクロ
    #[hooq::tag = "outer"]
    println!("{}", enresult(10)?);

    println!(
        "{}",
        #[hooq::tag = "inner"]
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

                    Err(())
                }
            };
        }

        #[hooq::tag = "outer"]
        id! {
            #[allow(unused)]
            fn outer() -> Result<(), ()> {
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

            #[allow(unused)]
            fn inner() -> Result<(), ()> {
                #[hooq::tag = "inner"]
                enresult(())?;

                Err(())
            }
        }
    }

    // Expr としてパースされるマクロ
    let _ = #[hooq::tag = "outer"]
    vec![enresult(10)?; enresult(2)?];

    let _ = vec![
        #[hooq::tag = "inner 1"]
        enresult(10)?,
        #[hooq::tag = "inner 2"]
        enresult(20)?,
        #[hooq::tag = "inner 3"]
        enresult(30)?,
    ];

    // 区切り文字に `,` と `;` の両方が混ざっているものの検証

    macro_rules! stmts_with_print {
        ($($s:stmt, $e:expr);*) => {
            $(
                $s
                println!("{}", $e);
            )*
        };
    }

    #[hooq::tag = "stmts_with_print"]
    stmts_with_print!(
        if enresult(true)? {
            println!("It's true");
        }, enresult("if let")?;
        for _ in enresult([1, 2])? {}, enresult("for loop")?
    );

    macro_rules! stmts_with_print_rev {
        ($($e:expr, $s:stmt);*) => {
            $(
                $s
                println!("{}", $e);
            )*
        };
    }

    #[hooq::tag = "stmts_with_print"]
    stmts_with_print_rev!(
        enresult("if let")?, if enresult(true)? {
            println!("It's true");
        };
        enresult("for loop")?, for _ in enresult([1, 2])? {}
    );

    macro_rules! vecs {
        ($($v:expr; $n:expr),*) => {
            vec![
                $(
                    vec![$v; $n]
                ),*
            ]
        };
    }

    #[hooq::tag = "vecs"]
    let _ = vecs![
        enresult(10)?; enresult(2)?,
        enresult(20)?; enresult(3)?
    ];

    // 空のマクロ呼び出し
    empty!();

    // Rustコードとして解釈できないものについては無理にパースしない
    // empty! と serde_json! でこのことを検証

    empty! {
        Hey! This string is not rust code but an English sentence! How do you feel?
    }

    let _ = serde_json::json!({
        "key": "value",
        "array": enresult([1, 2, 3])?,
    });

    Err(())
}

#[hooq]
#[hooq::hook_in_macros(false)]
fn no_hooks_to_macros() -> Result<(), ()> {
    let _ = vec![enresult(1)?, enresult(2)?, enresult(3)?];

    enresult(())?;

    println!("{}", enresult("beep")?);

    Err(())
}

#[test]
fn test() {
    func().unwrap_err();
    no_hooks_to_macros().unwrap_err();
}
