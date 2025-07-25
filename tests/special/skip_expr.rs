#![allow(unused_braces)]

use hooq::hooq;

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
    {
        println!("sub scope in expr");

        if enresult(false)? {
            #[hooq::skip]
            return Ok({ enresult(enresult(()))? })?;
        }
    };

    Ok(())
}

#[hooq]
#[hooq::method(.inspect(|_| {
    println!("tag: {}", $tag);
}))]
fn skip_item() -> Result<(), ()> {
    // item fn は別な箇所で検証済みなので飛ばす

    struct S;

    #[hooq::tag("impl")]
    #[hooq::skip] // nop
    impl S {
        // ここから
    }

    Ok(())
}

#[test]
fn test() {
    skip_stmts().unwrap();
}
