#![allow(unused_braces)]
#![allow(clippy::declare_interior_mutable_const)]
#![allow(clippy::let_unit_value)]

use hooq::hooq;
use std::sync::LazyLock;

#[hooq]
fn enresult<T>(t: T) -> Result<T, ()> {
    Ok(t)
}

#[hooq]
#[hooq::method(.inspect(|_| {
    println!("tag: {}", $tag);
}))]
fn skip_stmts() -> Result<(), ()> {
    #[hooq::tag("local")]
    #[hooq::skip]
    let _ = enresult(true)? && {
        #[hooq::tag("sub scope in local")]
        enresult(true)?
    };

    #[hooq::tag("item")]
    #[hooq::skip] // nop
    fn _fnc() -> Result<(), ()> {
        #[hooq::tag("sub scope in item")]
        enresult(())?;

        if true {
            return Ok(());
        }

        Ok(())
    }

    #[hooq::tag("expr")]
    #[hooq::skip]
    {
        println!("sub scope in expr");

        if enresult(false)? {
            #[hooq::skip]
            return Ok({ enresult(enresult(()))? })?;
        }

        enresult(())
    }?;

    Ok(())
}

#[hooq]
#[hooq::method(.inspect(|_| {
    println!("tag: {}", $tag);
}))]
fn skip_item() -> Result<(), ()> {
    // item fn は別な箇所で検証済みなので飛ばす

    struct _S;

    #[hooq::tag("impl")]
    #[hooq::skip] // nop
    impl _S {
        #[hooq::tag("sub scope in impl")]
        fn _method() -> Result<(), ()> {
            Ok(())
        }

        #[hooq::tag("sub scope in impl")]
        #[hooq::skip] // nop
        fn _method2() -> Result<(), ()> {
            Ok(())
        }
    }

    #[hooq::tag("mod")]
    #[hooq::skip] // nop
    mod m {
        #[hooq::tag("sub scope in mod")]
        fn _mod_fn() -> Result<(), ()> {
            Ok(())
        }

        #[hooq::tag("sub scope in mod")]
        #[hooq::skip] // nop
        fn _mod_fn2() -> Result<(), ()> {
            Ok(())
        }
    }

    #[hooq::tag("const")]
    #[hooq::skip] // nop
    const _C: LazyLock<u32> = LazyLock::new(|| {
        (|| -> Result<u32, ()> {
            #[hooq::tag("sub scope in const")]
            let res = enresult(42_u32)?;

            Ok(res)
        })()
        .unwrap_or(0)
    });

    #[hooq::tag("static")]
    #[hooq::skip] // nop
    static _SS: LazyLock<u32> = LazyLock::new(|| {
        (|| -> Result<u32, ()> {
            #[hooq::tag("sub scope in static")]
            let res = enresult(42_u32)?;

            Ok(res)
        })()
        .unwrap_or(0)
    });

    #[hooq::tag("trait")]
    #[hooq::skip] // nop
    trait _T {
        #[hooq::tag("sub scope in trait")]
        fn _trait_method() -> Result<(), ()> {
            Ok(())
        }

        #[hooq::tag("sub scope in trait")]
        #[hooq::skip] // nop
        fn _trait_method2() -> Result<(), ()> {
            Ok(())
        }
    }

    Ok(())
}

#[hooq]
#[hooq::method(.inspect(|_| {
    println!("tag: {}", $tag);
}))]
fn skip_expr() -> Result<(), ()> {
    #[hooq::tag("try")]
    let _ = {
        #[hooq::skip]
        enresult(())?
    };

    let _ = {
        #[hooq::tag("try")]
        #[hooq::skip]
        enresult((enresult(())?, {
            #[hooq::tag("sub scope in function args.")]
            enresult(())?
        }))?
    };

    #[hooq::tag("return")]
    let _f = |f: bool| -> Result<((), ()), ()> {
        if f {
            #[hooq::skip]
            return Ok((enresult(())?, { enresult(())? }));
        }
        Ok(((), ()))
    };

    #[hooq::tag("array")]
    #[hooq::skip]
    let _ = [enresult(1)?, enresult(2)?, {
        #[hooq::tag("sub scope in array")]
        enresult(3)?
    }];

    // asign は検証方法が思いつかなかったので省略

    // async & await は別な非同期関数にて検証

    // binary も検証方法が思いつかなかったためパス

    #[hooq::tag("block")]
    let _ = #[hooq::skip]
    {
        let _ = enresult(enresult(())?)?;

        enresult(enresult(enresult(()))?)?
    }?;

    #[hooq::tag("call")]
    #[hooq::skip]
    enresult(enresult(enresult({
        #[hooq::tag("sub scope in call")]
        enresult(())?
    })?)?)?;

    #[hooq::tag("call-2")]
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

    #[hooq::tag("cast")]
    let _ = #[hooq::skip]
    (enresult(10)?, { enresult(())? }).0 as u64;

    let _ = #[hooq::tag("closure")]
    #[hooq::skip]
    |f: bool| -> Result<(), ()> {
        #[hooq::tag("sub scope in closure")]
        enresult(())?;

        let _ = enresult(enresult(())?)?;

        if f {
            return Ok(());
        }

        enresult(enresult(enresult(()))?)?
    };

    #[hooq::tag("const block")]
    #[hooq::skip]
    const _C: () = const {
        let _ = || {
            #[hooq::tag("sub scope in const block")]
            enresult(())?;

            Result::<(), ()>::Ok(())
        };
    };

    struct Strct {
        #[allow(unused)]
        field: (),
    }

    // 順番的には field の検証が先で、
    // Struct の検証は後ろだが、似ているためここで検証

    let _ = #[hooq::tag("struct")]
    #[hooq::skip]
    Strct {
        field: ((enresult(())?, {
            #[hooq::tag("sub scope in struct field")]
            enresult(())?
        })
            .0),
    };

    let _ = #[hooq::tag("field")]
    Strct {
        #[hooq::skip]
        field: ((enresult(())?, {
            #[hooq::tag("sub scope in struct field")]
            enresult(())?
        })
            .0),
    };

    #[hooq::tag("for_loop")]
    #[hooq::skip]
    for _ in enresult([{
        #[hooq::tag("sub scope in for loop expr")]
        enresult(())?
    }])? {
        #[hooq::tag("sub scope in for loop")]
        enresult(())?;
    }

    // Group は ExprParen と同様の内容なので省略

    #[hooq::tag("if")]
    #[hooq::skip]
    if enresult({
        #[hooq::tag("sub scope in if condition")]
        enresult(true)?
    })? {
        #[hooq::tag("sub scope in if")]
        enresult(())?;
    // ここは skip が効かないがやむを得ない
    } else if enresult({
        #[hooq::tag("sub scope in if condition")]
        enresult(true)?
    })? {
        #[hooq::tag("sub scope in else if")]
        enresult(())?;
    } else {
        #[hooq::tag("sub scope in else")]
        enresult(())?;
    };

    let _ = #[hooq::tag("index")]
    #[hooq::skip]
    enresult([enresult(())?, { enresult(())? }])?[(enresult(0)?, { enresult(())? }).0];

    #[hooq::tag("let")]
    if let Ok(()) = enresult(())
        && #[hooq::skip]
        let Ok(()) = enresult((enresult(()), { enresult(())? }).0)?
    {
        enresult(())?;
    }

    let mut i = 0;

    let _ = #[hooq::tag("loop")]
    #[hooq::skip]
    loop {
        if i > 0 {
            break enresult(());
        }

        i += 1;

        enresult(())?;
    }?;

    #[allow(clippy::unit_arg)]
    let _ = #[hooq::tag("match")]
    #[hooq::skip]
    match enresult(Some({
        #[hooq::tag("sub scope in match expr")]
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
        None => {
            enresult(())?;
            enresult(())
        }
    }?;

    impl Strct {
        fn method(&self, _: ()) -> Result<&Self, ()> {
            Ok(self)
        }
    }

    let s = Strct { field: () };

    #[hooq::tag("method_call")]
    #[hooq::skip]
    s.method(enresult(())?)?.method({
        #[hooq::tag("sub scope in method call")]
        enresult(())?;

        enresult(())?
    })?;

    let _ = #[hooq::tag("paren")]
    #[hooq::skip]
    (enresult({ enresult(enresult(()))? })?)?;

    let _ = #[hooq::tag("range")]
    #[hooq::skip]
    enresult(enresult(0)?)?..enresult({ enresult(1)? })?;

    // rawAddr, reference は実質使われないため検証不要

    let _ = #[hooq::tag("repeat")]
    #[hooq::skip]
    [enresult((enresult(0)?, { enresult(())? }).0)?; {
        fn _f() -> Result<(), ()> {
            enresult(())?;

            Ok(())
        }

        5
    }];

    // struct は先に検証済み

    // try block は nightly な機能と考えられるのでテスト省略

    let _ = #[hooq::tag("tuple")]
    #[hooq::skip]
    (enresult(())?, {
        #[hooq::tag("sub scope in tuple")]
        enresult(())?
    });

    let _ = #[hooq::tag("unary")]
    #[hooq::skip]
    !enresult((enresult(true)?, { enresult(())? }).0)?;

    let _ = #[hooq::tag("unsafe")]
    #[hooq::skip]
    unsafe {
        unsafe fn unsafe_enresult<T>(value: T) -> Result<T, ()> {
            Ok(value)
        }

        #[hooq::tag("sub scope in unsafe")]
        unsafe_enresult(())?;

        unsafe_enresult(())
    }?;

    #[hooq::tag("while")]
    #[hooq::skip]
    while (enresult(false)?, {
        enresult(())?;
    })
        .0
    {
        #[hooq::tag("sub scope in while")]
        enresult(())?;
    }

    // yield は検証方法が思いつかなかったためパス

    Ok(())
}

#[hooq]
#[hooq::method(.inspect(|_| {
    println!("tag: {}", $tag);
}))]
async fn skip_expr_async_await() -> Result<(), ()> {
    #[hooq::tag("async & await")]
    #[hooq::skip]
    let _ = async {
        #[hooq::tag("sub scope in async")]
        enresult(async {})?.await;

        #[hooq::skip]
        enresult(async {})?.await;

        Ok(())
    }
    .await?;

    Ok(())
}

#[tokio::test]
async fn test() {
    skip_stmts().unwrap();
    skip_item().unwrap();
    skip_expr().unwrap();
    skip_expr_async_await().await.unwrap();
}
