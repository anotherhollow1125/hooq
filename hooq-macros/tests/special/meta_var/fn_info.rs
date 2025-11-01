use hooq_macros::hooq;

fn enresult<T>(t: T) -> Result<T, ()> {
    Ok(t)
}

#[allow(dead_code)]
type Unit = ();

#[hooq]
#[hooq::method(.inspect_err(|_| {
    eprintln!("
fn_name: {}
fn_sig:  {}", $fn_name, $fn_sig);
}))]
mod tmp {
    use std::sync::LazyLock;

    use super::{Unit, enresult};

    static _TMP: LazyLock<Result<(), ()>> = LazyLock::new(|| {
        enresult(())?;

        let _ = || -> Result<(), ()> {
            enresult(())?;

            fn _inner() {
                let _ = || -> Result<Unit, Unit> {
                    enresult(())?;
                    Ok(())
                };
            }

            enresult(())?;

            Ok(())
        };

        enresult(())?;

        Ok(())
    });
}

#[hooq]
#[hooq::method(.inspect_err(|_| {
    eprintln!("
fn_name: {}
fn_sig:  {}", $fn_name, $fn_sig);
}))]
fn func() -> Result<(), ()> {
    enresult(())?;

    let _ = || -> Result<(), ()> {
        enresult(())?;

        fn _inner() -> Result<(), Unit> {
            enresult(())?;

            let _ = || -> Result<Unit, Unit> {
                enresult(())?;

                let _ = |(): Unit| -> Result<Unit, Unit> {
                    enresult(())?;

                    let _ = |(): Unit, (): Unit| -> Result<Unit, Unit> {
                        enresult(())?;

                        let _ = |((), (), ()): (Unit, Unit, Unit)| -> Result<Unit, Unit> {
                            enresult(())?;

                            Ok(())
                        };

                        Ok(())
                    };

                    Ok(())
                };

                Ok(())
            };
            enresult(())?;

            Ok(())
        }
        enresult(())?;

        Ok(())
    };
    enresult(())?;

    Ok(())
}

#[test]
fn test_fn_info() {
    func().unwrap();
}
