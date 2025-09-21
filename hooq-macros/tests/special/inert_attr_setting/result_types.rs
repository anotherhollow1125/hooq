use hooq_macros::hooq;

#[hooq]
#[hooq::tail_expr_idents()]
#[hooq::result_types("Result", "Either")]
mod funcs {
    type Either = Result<(), ()>;
    type NotTarget = Result<(), ()>;

    fn enresult<T>(val: T) -> Result<T, ()> {
        Ok(val)
    }

    pub fn result_fn() -> Result<(), ()> {
        if enresult(false)? {
            return enresult(());
        }

        enresult(())
    }

    pub fn either_fn() -> Either {
        if enresult(false)? {
            return enresult(());
        }

        enresult(())
    }

    pub fn other_fn_1() -> NotTarget {
        if enresult(false)? {
            return enresult(());
        }

        Err(())
    }

    // result_types に含まれていなくても、
    // tail_expr_idents に含まれていれば
    // tail_expr_idents の方にはフックする

    #[hooq::tail_expr_idents("Ok")]
    #[hooq::not_tail_expr_idents()]
    pub fn other_fn_2() -> NotTarget {
        if enresult(false)? {
            return enresult(());
        }

        if enresult(false)? {
            return Ok(());
        }

        Ok(())
    }

    pub fn other_fn_3() -> NotTarget {
        if enresult(false)? {
            return enresult(());
        }

        if enresult(false)? {
            // 今度はこちらにはフックせず
            return Err(());
        }

        #[hooq::tail_expr_idents("Ok")]
        Ok(())
    }
}

#[test]
fn test() {
    funcs::result_fn().unwrap();
    funcs::either_fn().unwrap();
    funcs::other_fn_1().unwrap_err();
    funcs::other_fn_2().unwrap();
    funcs::other_fn_3().unwrap();
}
