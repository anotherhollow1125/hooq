#![allow(unused_braces)]
#![allow(clippy::declare_interior_mutable_const)]
#![allow(clippy::let_unit_value)]

use std::sync::LazyLock;

use hooq_macros::hooq;
use util_macros::id;

#[hooq]
fn enresult<T>(t: T) -> Result<T, ()> {
    Ok(t)
}

#[hooq]
#[hooq::method(.inspect(|_| {
    println!("tag: {}", $tag);
}))]
#[hooq::tag = "(no tag)"]
#[hooq::tail_expr_idents("Ok", "Err")]
fn skip_stmts() -> Result<(), ()> {
    #[hooq::tag = "local"]
    #[hooq::skip]
    let _ = enresult(true)? && {
        #[hooq::tag = "sub scope in local"]
        enresult(true)?
    };

    #[hooq::tag = "item"]
    #[hooq::skip] // nop
    fn _fnc() -> Result<(), ()> {
        #[hooq::tag = "sub scope in item"]
        enresult(())?;

        if true {
            return Err(());
        }

        Err(())
    }

    #[hooq::tag = "expr"]
    #[hooq::skip]
    {
        println!("sub scope in expr");

        if enresult(false)? {
            #[hooq::skip]
            return Ok({ enresult(enresult(()))? })?;
        }

        enresult(())
    }?;

    #[hooq::tag = "macro"]
    #[hooq::skip]
    println!("{}, {}", enresult(10)?, {
        enresult(())?;

        Result::<u32, ()>::Ok(20)
    }?);

    #[hooq::tag = "macro"]
    println!(
        "{}, {}",
        enresult(10)?,
        #[hooq::skip]
        {
            enresult(())?;

            Result::<u32, ()>::Ok(20)
        }?
    );

    Err(())
}

#[hooq]
#[hooq::method(.inspect(|_| {
    println!("tag: {}", $tag);
}))]
#[hooq::tag = "(no tag)"]
#[hooq::tail_expr_idents("Ok", "Err")]
fn skip_item() -> Result<(), ()> {
    // item fn は別な箇所で検証済みなので飛ばす

    struct _S;

    #[hooq::tag = "impl"]
    #[hooq::skip] // nop
    impl _S {
        #[hooq::tag = "sub scope in impl"]
        fn _method() -> Result<(), ()> {
            Err(())
        }

        #[hooq::tag = "sub scope in impl"]
        #[hooq::skip] // nop
        fn _method2() -> Result<(), ()> {
            Err(())
        }

        #[hooq::tag = "sub scope in impl"]
        #[hooq::skip] // nop
        id! {
            fn _method3(&self) -> Result<(), ()> {
                Err(())
            }
        }

        #[hooq::tag = "sub scope in impl"]
        id! {
            #[hooq::skip] // nop
            fn _method4(&self) -> Result<(), ()> {
                Err(())
            }
        }
    }

    #[hooq::tag = "mod"]
    #[hooq::skip] // nop
    mod m {
        #[hooq::tag = "sub scope in mod"]
        fn _mod_fn() -> Result<(), ()> {
            Err(())
        }

        #[hooq::tag = "sub scope in mod"]
        #[hooq::skip] // nop
        fn _mod_fn2() -> Result<(), ()> {
            Err(())
        }
    }

    #[hooq::tag = "const"]
    #[hooq::skip] // nop
    const _C: LazyLock<u32> = LazyLock::new(|| {
        (|| -> Result<u32, ()> {
            #[hooq::tag = "sub scope in const"]
            let res = enresult(42_u32)?;

            Ok(res)
        })()
        .unwrap_or(0)
    });

    #[hooq::tag = "static"]
    #[hooq::skip] // nop
    static _SS: LazyLock<u32> = LazyLock::new(|| {
        (|| -> Result<u32, ()> {
            #[hooq::tag = "sub scope in static"]
            let res = enresult(42_u32)?;

            Ok(res)
        })()
        .unwrap_or(0)
    });

    #[hooq::tag = "trait"]
    #[hooq::skip] // nop
    trait _T {
        #[hooq::tag = "sub scope in trait"]
        fn _trait_method() -> Result<(), ()> {
            Err(())
        }

        #[hooq::tag = "sub scope in trait"]
        #[hooq::skip] // nop
        fn _trait_method2() -> Result<(), ()> {
            Err(())
        }

        #[hooq::tag = "sub scope in impl"]
        #[hooq::skip] // nop
        id! {
            fn _trait_method3(&self) -> Result<(), ()> {
                Err(())
            }
        }

        #[hooq::tag = "sub scope in impl"]
        id! {
            #[hooq::skip] // nop
            fn _trait_method4(&self) -> Result<(), ()> {
                Err(())
            }
        }
    }

    mod tmp {
        use super::*;

        #[hooq::tag = "macro"]
        #[hooq::skip] // nop
        id! {
            fn _macro_fn() -> Result<(), ()> {
                Err(())
            }
        }

        #[hooq::tag = "macro"]
        id! {
            #[allow(clippy::needless_question_mark)]
            fn _macro_fn_2() -> Result<(), ()> {
                #[hooq::skip]
                Ok({
                    enresult(())?;

                    Result::<(), ()>::Err(())
                }?)
            }
        }
    }

    Err(())
}

#[hooq]
#[hooq::method(.inspect(|_| {
    println!("tag: {}", $tag);
}))]
#[hooq::tag = "(no tag)"]
#[hooq::tail_expr_idents("Ok", "Err")]
fn skip_expr() -> Result<(), ()> {
    #[hooq::tag = "try"]
    let _ = {
        #[hooq::skip]
        enresult(())?
    };

    let _ = {
        #[hooq::tag = "try"]
        #[hooq::skip]
        enresult((enresult(())?, {
            #[hooq::tag = "sub scope in function args."]
            enresult(())?
        }))?
    };

    #[hooq::tag = "return"]
    let _f = |f: bool| -> Result<((), ()), ()> {
        if f {
            #[hooq::skip]
            return Ok((enresult(())?, { enresult(())? }));
        }
        Ok(((), ()))
    };

    #[hooq::tag = "array"]
    #[hooq::skip]
    let _ = [enresult(1)?, enresult(2)?, {
        #[hooq::tag = "sub scope in array"]
        enresult(3)?
    }];

    // asign は検証方法が思いつかなかったので省略

    // async & await は別な非同期関数にて検証

    // binary も検証方法が思いつかなかったためパス

    #[hooq::tag = "block"]
    let _ = #[hooq::skip]
    {
        let _ = enresult(enresult(())?)?;

        enresult(enresult(enresult(()))?)?
    }?;

    #[hooq::tag = "call"]
    #[hooq::skip]
    enresult(enresult(enresult({
        #[hooq::tag = "sub scope in call"]
        enresult(())?
    })?)?)?;

    #[hooq::tag = "call-2"]
    #[hooq::skip]
    ((
        enresult(|| {
            enresult(())?;

            enresult(())
        })?,
        {
            enresult(())?;
        },
    )
        .0)()?;

    #[hooq::tag = "cast"]
    let _ = #[hooq::skip]
    (enresult(10)?, { enresult(())? }).0 as u64;

    let _ = #[hooq::tag = "closure"]
    #[hooq::skip] // nop
    |f: bool| -> Result<(), ()> {
        #[hooq::tag = "sub scope in closure"]
        enresult(())?;

        let _ = enresult(enresult(())?)?;

        if f {
            return Err(());
        }

        enresult(enresult(enresult(()))?)?
    };

    #[hooq::tag = "const block"]
    #[hooq::skip] // nop
    const _C: () = const {
        let _ = || {
            #[hooq::tag = "sub scope in const block"]
            enresult(())?;

            Result::<(), ()>::Err(())
        };
    };

    struct Strct {
        #[allow(unused)]
        field: (),
        #[allow(unused)]
        field2: (),
    }

    // 順番的には field の検証が先で、
    // Struct の検証は後ろだが、似ているためここで検証

    let _ = #[hooq::tag = "struct"]
    #[hooq::skip]
    Strct {
        field: ((enresult(())?, {
            #[hooq::tag = "sub scope in struct field"]
            enresult(())?
        })
            .0),
        field2: ((enresult(())?, {
            #[hooq::tag = "sub scope in struct field"]
            enresult(())?
        })
            .0),
    };

    let _ = #[hooq::tag = "field"]
    Strct {
        #[hooq::skip]
        field: ((enresult(())?, {
            #[hooq::tag = "sub scope in struct field"]
            enresult(())?
        })
            .0),
        field2: ((enresult(())?, {
            #[hooq::tag = "sub scope in struct field"]
            enresult(())?
        })
            .0),
    };

    #[hooq::tag = "for_loop"]
    #[hooq::skip]
    for _ in enresult([{
        #[hooq::tag = "sub scope in for loop expr"]
        enresult(())?
    }])? {
        #[hooq::tag = "sub scope in for loop"]
        enresult(())?;
    }

    // Group は ExprParen と同様の内容なので省略

    #[hooq::tag = "if"]
    #[hooq::skip]
    if enresult({
        #[hooq::tag = "sub scope in if condition"]
        enresult(true)?
    })? {
        #[hooq::tag = "sub scope in if"]
        enresult(())?;
    // ここは skip が効かないがやむを得ない
    } else if enresult({
        #[hooq::tag = "sub scope in if condition"]
        enresult(true)?
    })? {
        #[hooq::tag = "sub scope in else if"]
        enresult(())?;
    } else {
        #[hooq::tag = "sub scope in else"]
        enresult(())?;
    };

    let _ = #[hooq::tag = "index"]
    #[hooq::skip]
    enresult([enresult(())?, { enresult(())? }])?[(enresult(0)?, { enresult(())? }).0];

    #[hooq::tag = "let"]
    if let Err(()) = enresult(())
        && #[hooq::skip]
        let Err(()) = enresult((enresult(enresult(()))?, { enresult(())? }).0)?
    {
        enresult(())?;
    }

    let mut i = 0;

    let _ = #[hooq::tag = "loop"]
    #[hooq::skip]
    loop {
        if i > 0 {
            break enresult(());
        }

        i += 1;

        enresult(())?;
    }?;

    let _ = #[hooq::tag = "macro-outer"]
    #[hooq::skip]
    vec![enresult(0)?, enresult(1)?, {
        #[hooq::tag = "sub scope in macro-outer"]
        enresult(2)?
    }];

    let _ = #[hooq::tag = "macro-inner"]
    vec![
        #[hooq::skip]
        vec![enresult(0)?, enresult(1)?, {
            #[hooq::tag = "sub scope in macro-inner"]
            enresult(2)?
        }],
    ];

    #[allow(clippy::unit_arg)]
    let _ = #[hooq::tag = "match"]
    #[hooq::skip]
    match enresult(Some({
        #[hooq::tag = "sub scope in match expr"]
        enresult(())?
    }))? {
        Some(()) if enresult(true)? => {
            enresult(())?;
            enresult(())
        }
        Some(()) => {
            enresult(())?;
            enresult(())
        }
        None => enresult(()),
    }?;

    impl Strct {
        fn method(&self, _: ()) -> Result<&Self, ()> {
            Ok(self)
        }
    }

    let s = Strct {
        field: (),
        field2: (),
    };

    #[hooq::tag = "method_call"]
    #[hooq::skip]
    s.method(enresult(())?)?.method({
        #[hooq::tag = "sub scope in method call"]
        enresult(())?;

        enresult(())?
    })?;

    let _ = #[hooq::tag = "paren"]
    #[hooq::skip]
    (enresult({ enresult(enresult(()))? })?)?;

    // Range 全体に skip を適用することは難しいため、
    // range-1, range-2 のテストはあまり意味がない

    let _ = #[hooq::tag = "range-1"]
    #[hooq::skip]
    enresult(
        (enresult(0)?, {
            enresult(())?;
        })
            .0,
    )?
        ..enresult(
            (enresult(1)?, {
                enresult(())?;
            })
                .0,
        )?;

    let _ = #[hooq::tag = "range-2"]
    enresult(
        (enresult(0)?, {
            enresult(())?;
        })
            .0,
    )?
        ..(#[hooq::skip]
        enresult(
            (enresult(1)?, {
                enresult(())?;
            })
                .0,
        )?);

    // rawAddr, reference は実質使われないため検証不要

    let _ = #[hooq::tag = "repeat"]
    #[hooq::skip]
    [enresult((enresult(0)?, { enresult(())? }).0)?; {
        fn _f() -> Result<(), ()> {
            enresult(())?;

            Err(())
        }

        5
    }];

    // struct は先に検証済み

    // try block は nightly な機能と考えられるのでテスト省略

    let _ = #[hooq::tag = "tuple"]
    #[hooq::skip]
    (enresult(())?, {
        #[hooq::tag = "sub scope in tuple"]
        enresult(())?
    });

    let _ = #[hooq::tag = "unary"]
    #[hooq::skip]
    !enresult((enresult(true)?, { enresult(())? }).0)?;

    let _ = #[hooq::tag = "unsafe"]
    #[hooq::skip]
    unsafe {
        unsafe fn unsafe_enresult<T>(value: T) -> Result<T, ()> {
            Ok(value)
        }

        #[hooq::tag = "sub scope in unsafe"]
        unsafe_enresult(())?;

        unsafe_enresult(())
    }?;

    #[hooq::tag = "while"]
    #[hooq::skip]
    while (enresult(false)?, {
        enresult(())?;
    })
        .0
    {
        #[hooq::tag = "sub scope in while"]
        enresult(())?;
    }

    // yield は検証方法が思いつかなかったためパス

    Err(())
}

#[hooq]
#[hooq::method(.inspect(|_| {
    println!("tag: {}", $tag);
}))]
#[hooq::tag = "(no tag)"]
#[hooq::tail_expr_idents("Ok", "Err")]
async fn skip_expr_async_await() -> Result<(), ()> {
    #[hooq::tag = "async & await"]
    #[hooq::skip]
    let _ = async {
        #[hooq::tag = "sub scope in async"]
        enresult(async {
            enresult(async {})?.await;
            enresult(())
        })?
        .await?;

        #[hooq::skip]
        enresult(async {
            enresult(())?;
            enresult(())
        })?
        .await?;

        Err(())
    }
    .await?;

    Err(())
}

#[hooq]
#[hooq::method(.inspect(|_| {
    println!("tag: {}", $tag);
}))]
#[hooq::tail_expr_idents("Ok", "Err")]
fn skip_last_ok() -> Result<(), ()> {
    let _: Result<(), ()> = #[hooq::tag = "last ok 1"]
    #[hooq::skip]
    {
        Err(())
    };

    let _: Result<(), ()> = #[hooq::tag = "last ok 2"]
    {
        #[hooq::skip]
        Err(())
    };

    #[hooq::tag = "last ok 3"]
    #[hooq::skip]
    Err(())
}

#[tokio::test]
async fn test() {
    skip_stmts().unwrap_err();
    skip_item().unwrap_err();
    skip_expr().unwrap_err();
    skip_expr_async_await().await.unwrap_err();
    skip_last_ok().unwrap_err();
}
