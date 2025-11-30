use std::fmt::Debug;

use hooq::hooq;

#[hooq]
#[hooq::method(.my_inspect())]
#[hooq::tail_expr_idents("Some")]
#[hooq::ignore_tail_expr_idents("None")]
#[hooq::result_types("Option")]
fn main() {
    fn option_fn_1() -> Option<i32> {
        // hook target
        Some(42)
    }

    fn option_fn_2<T: Debug>(flag: bool, val: T) -> Option<T> {
        // hook target
        let _ = option_fn_1()?;

        // hook target because the return type of the function is Option
        if flag {
            // hook target
            Some(val)
        } else {
            // NOT hook target
            None
        }
    }

    fn result_fn_1() -> Result<i32, ()> {
        // NOT hook target because the return type of the function is Result not Option
        Ok(42)
    }

    fn result_fn_2() -> Result<i32, ()> {
        // HOOK TARGET because of `?`
        // so, #[hooq::skip_all] is needed
        #[hooq::skip_all]
        let _ = result_fn_1()?;

        // NOT hook target because the return type of the function is Result not Option
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
                println!("Inspecting value: {:?}", val);
                Some(val)
            }
            None => None,
        }
    }
}
