use hooq_macros::hooq;
fn fire<T>(val: T, line: usize) -> T {
    {
        ::std::io::_print(format_args!("ファイア! @ {0}\n", line));
    };
    val
}
fn ice_storm<T>(val: T, line: usize) -> T {
    {
        ::std::io::_print(format_args!("アイスストーム! @ {0}\n", line));
    };
    val
}
fn brain_damned<T>(val: T, line: usize) -> T {
    {
        ::std::io::_print(format_args!("ブレインダムド! @ {0}\n", line));
    };
    val
}
fn jugemu<T>(val: T, line: usize) -> T {
    {
        ::std::io::_print(format_args!("ジュゲム! @ {0}\n", line));
    };
    val
}
fn bayoeen<T>(val: T, line: usize) -> T {
    {
        ::std::io::_print(format_args!("ばよえ～ん! @ {0}\n", line));
    };
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
fn func() -> Result<(), ()> {
    enresult(()).fire(55usize)?;
    {
        enresult(()).fire(59usize).ice_storm(59usize)?;
        {
            enresult(()).fire(63usize).ice_storm(63usize).brain_damned(63usize)?;
            {
                enresult(())
                    .fire(67usize)
                    .ice_storm(67usize)
                    .brain_damned(67usize)
                    .jugemu(67usize)?;
                enresult(())
                    .fire(70usize)
                    .ice_storm(70usize)
                    .brain_damned(70usize)
                    .jugemu(70usize)
                    .bayoeen(70usize)?;
                enresult(())
                    .bayoeen(73usize)
                    .fire(73usize)
                    .ice_storm(73usize)
                    .brain_damned(73usize)
                    .jugemu(73usize)?;
            }
            enresult(())
                .jugemu(77usize)
                .fire(77usize)
                .ice_storm(77usize)
                .brain_damned(77usize)?;
        }
        enresult(()).brain_damned(81usize).fire(81usize).ice_storm(81usize)?;
    }
    enresult(()).ice_storm(85usize).fire(85usize)?;
    enresult(()).fire(87usize)?;
    Ok(())
}
fn func2() -> Result<(), ()> {
    fire(enresult(()), 95usize)?;
    {
        ice_storm(fire(enresult(()), 99usize), 99usize)?;
        {
            brain_damned(ice_storm(fire(enresult(()), 103usize), 103usize), 103usize)?;
            {
                jugemu(
                    brain_damned(
                        ice_storm(fire(enresult(()), 107usize), 107usize),
                        107usize,
                    ),
                    107usize,
                )?;
                bayoeen(
                    jugemu(
                        brain_damned(
                            ice_storm(fire(enresult(()), 110usize), 110usize),
                            110usize,
                        ),
                        110usize,
                    ),
                    110usize,
                )?;
            }
            brain_damned(ice_storm(fire(enresult(()), 113usize), 113usize), 113usize)?;
        }
        ice_storm(fire(enresult(()), 116usize), 116usize)?;
    }
    fire(enresult(()), 119usize)?;
    Ok(())
}
fn func3() -> Result<(), ()> {
    enresult(()).fire(127usize)?;
    {
        ice_storm(enresult(()).fire(131usize), 131usize)?;
        {
            ice_storm(enresult(()).fire(135usize), 135usize).brain_damned(135usize)?;
            {
                ice_storm(enresult(()).fire(139usize), 139usize)
                    .brain_damned(139usize)
                    .jugemu(139usize)?;
                bayoeen(
                    ice_storm(enresult(()).fire(142usize), 142usize)
                        .brain_damned(142usize)
                        .jugemu(142usize),
                    142usize,
                )?;
            }
            ice_storm(enresult(()).fire(145usize), 145usize).brain_damned(145usize)?;
        }
    }
    Ok(())
}
