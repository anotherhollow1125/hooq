use hooq_macros::hooq;

fn fire<T>(val: T, line: usize) -> T {
    println!("ファイア! @ {line}");
    val
}

fn ice_storm<T>(val: T, line: usize) -> T {
    println!("アイスストーム! @ {line}");
    val
}

fn brain_damned<T>(val: T, line: usize) -> T {
    println!("ブレインダムド! @ {line}");
    val
}

fn jugemu<T>(val: T, line: usize) -> T {
    println!("ジュゲム! @ {line}");
    val
}

fn bayoeen<T>(val: T, line: usize) -> T {
    println!("ばよえ～ん! @ {line}");
    val
}

trait Magics: Sized {
    fn fire(self, line: usize) -> Self {
        fire(self, line)
    }
    fn ice_storm(self, line: usize) -> Self {
        ice_storm(self, line)
    }
    fn brain_damned(self, line: usize) -> Self {
        brain_damned(self, line)
    }
    fn jugemu(self, line: usize) -> Self {
        jugemu(self, line)
    }
    fn bayoeen(self, line: usize) -> Self {
        bayoeen(self, line)
    }
}

impl<T: Sized> Magics for T {}

fn enresult<T>(val: T) -> Result<T, ()> {
    Ok(val)
}

#[hooq]
#[hooq::method(.fire($line))]
fn func() -> Result<(), ()> {
    enresult(())?;

    #[hooq::method(.$so_far.ice_storm($line))]
    {
        enresult(())?;

        #[hooq::method(.$so_far.brain_damned($line))]
        {
            enresult(())?;

            #[hooq::method(.$so_far.jugemu($line))]
            {
                enresult(())?;

                #[hooq::method(.$so_far.bayoeen($line))]
                enresult(())?;

                #[hooq::method(.bayoeen($line).$so_far)]
                enresult(())?;
            }

            #[hooq::method(.jugemu($line).$so_far)]
            enresult(())?;
        }

        #[hooq::method(.brain_damned($line).$so_far)]
        enresult(())?;
    }

    #[hooq::method(.ice_storm($line).$so_far)]
    enresult(())?;

    enresult(())?;

    Ok(())
}

#[hooq]
#[hooq::method(fire($expr, $line))]
fn func2() -> Result<(), ()> {
    enresult(())?;

    #[hooq::method(ice_storm($so_far, $line))]
    {
        enresult(())?;

        #[hooq::method(brain_damned($so_far, $line))]
        {
            enresult(())?;

            #[hooq::method(jugemu($so_far, $line))]
            {
                enresult(())?;

                #[hooq::method(bayoeen($so_far, $line))]
                enresult(())?;
            }

            enresult(())?;
        }

        enresult(())?;
    }

    enresult(())?;

    Ok(())
}

#[hooq]
#[hooq::method(.fire($line))]
fn func3() -> Result<(), ()> {
    enresult(())?;

    #[hooq::method(ice_storm($expr.$so_far, $line))]
    {
        enresult(())?;

        #[hooq::method($so_far.brain_damned($line))]
        {
            enresult(())?;

            #[hooq::method($so_far.jugemu($line))]
            {
                enresult(())?;

                #[hooq::method(bayoeen($so_far, $line))]
                enresult(())?;
            }

            enresult(())?;
        }
    }

    Ok(())
}

#[test]
fn test_func() {
    func().unwrap();
    func2().unwrap();
    func3().unwrap();
}
