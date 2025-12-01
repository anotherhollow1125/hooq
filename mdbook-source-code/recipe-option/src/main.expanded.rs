#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use std::fmt::Debug;
use hooq::hooq;
fn main() {
    fn option_fn_1() -> Option<i32> {
        Some(42).my_inspect()
    }
    fn option_fn_2<T: Debug>(flag: bool, val: T) -> Option<T> {
        let _ = option_fn_1().my_inspect()?;
        if flag { Some(val).my_inspect() } else { None }.my_inspect()
    }
    fn result_fn_1() -> Result<i32, ()> {
        Ok(42)
    }
    fn result_fn_2() -> Result<i32, ()> {
        let _ = result_fn_1()?;
        Ok(42)
    }
    let _ = option_fn_1();
    let _ = option_fn_2(true, 123);
    let _ = result_fn_1();
    let _ = result_fn_2();
}
trait MyInspect {
    fn my_inspect(self) -> Self;
}
impl<T> MyInspect for Option<T>
where
    T: Debug,
{
    fn my_inspect(self) -> Self {
        match self {
            Some(val) => {
                {
                    ::std::io::_print(format_args!("Inspecting value: {0:?}\n", val));
                };
                Some(val)
            }
            None => None,
        }
    }
}
