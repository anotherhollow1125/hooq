#![allow(unused_braces)]

use hooq_macros::hooq;

#[hooq]
fn enresult<T>(t: T) -> Result<T, ()> {
    Ok(t)
}

#[hooq]
#[hooq::skip_all]
fn skip_target() -> Result<(), ()> {
    enresult(())?;

    #[hooq::tag = "this attribute must be removed"]
    if enresult(false)? {
        #[hooq::tag = "this attribute must be removed"]
        return Ok(());
    }

    if enresult(false)? {
        #[hooq::tag = "this attribute must be removed"]
        return enresult(());
    }

    #[hooq::tag = "this attribute must be removed"]
    let _: Result<(), ()> = {
        enresult(())?;

        #[hooq::tag = "this attribute must be removed"]
        if enresult(false)? {
            #[hooq::tag = "this attribute must be removed"]
            enresult(())?;

            return Ok(());
        }

        Ok(())
    };

    Ok(())
}

#[hooq]
#[hooq::method(.inspect(|_| {
    println!("tag: {}", $tag);
}))]
#[hooq::tail_expr_idents("Ok", "Err")]
fn complex_skip() -> Result<(), ()> {
    let gen_bools = || enresult(true);

    #[hooq::tag = "1"]
    if gen_bools()? {
        #[hooq::skip]
        #[hooq::tag = "2"]
        if gen_bools()? {
            if gen_bools()? {
                #[hooq::tag = "3"]
                enresult(())?;

                #[hooq::skip]
                if enresult(false)? {
                    enresult(())?;

                    return {
                        #[hooq::skip]
                        Ok(())
                    };
                }

                Ok(())
            } else {
                enresult(())?;

                #[hooq::skip_all]
                if enresult(false)? {
                    enresult(())?;

                    return Err(());
                }

                if enresult(false)? {
                    if enresult(false)? {
                        return { Err(()) };
                    }

                    #[hooq::skip]
                    return {
                        #[hooq::skip]
                        {
                            Err(())
                        }
                    };
                }

                Err(())
            }
        } else {
            Ok(())
        }
    } else {
        Ok(())
    }
}

#[hooq]
#[hooq::method(.inspect(|_| {
    println!("tag: {}", $tag);
}))]
#[hooq::tag = "(no tag)"]
fn func() -> Result<(), ()> {
    skip_target()?; // Not Skipped

    complex_skip()?;

    Ok(())
}

#[test]
fn test() {
    func().unwrap();
}
