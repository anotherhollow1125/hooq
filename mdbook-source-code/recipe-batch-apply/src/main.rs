use hooq::hooq;

#[hooq] // attribute macro root exist only here
#[hooq::method(.inspect_err(|_| {}))]
mod hoge {
    pub fn bar() -> i32 {
        // NOT hook target because this function does not return Result
        42
    }

    pub fn failable<T>(val: T) -> Result<T, ()> {
        // The return type of this function is Result
        // but "Ok" is specified as ignore_tail_expr_idents by default
        // so NOT hook target
        Ok(val)
    }

    #[hooq::tail_expr_idents("Ok", "Err")]
    pub fn _failable_2<T>(val: T) -> Result<T, ()> {
        if failable(false).unwrap() {
            // ↑ above is NOT hook target
            // ↓ below: hook target because the return type of this function is Result
            return failable(val);
        }

        // hook target because "Ok" is now specified as tail_expr_idents
        Ok(val)
    }

    pub fn fuga() -> Result<(), ()> {
        // hook target because of `?`
        failable(())?;

        let _ = || {
            // hook target because of `?`
            failable(())?;

            // hook target because of `?`
            if failable(false)? {
                // hook target because "Err" is specified as tail_expr_idents by default
                return Err(());
            }

            // NOT hook target because "Ok" is specified as ignore_tail_expr_idents by default
            Ok(())
        };

        let _ = || {
            // NOT hook target because "Ok" is specified as ignore_tail_expr_idents by default
            Result::<(), ()>::Ok(())
        };

        let _ = {
            let _ = bar();

            // hook target because "Err" is specified as tail_expr_idents by default
            // even if this is the tail expression of the block not a closure
            Result::<(), ()>::Err(())
        };

        let _ = || -> Result<(), ()> {
            // hook target because hooq can know this closure returns Result
            failable(())
        };

        let _ = || {
            // NOT hook target because hooq cannot know this closure returns Result
            failable(())
        };

        // NOT hook target because "Ok" is specified as ignore_tail_expr_idents by default
        Ok(())
    }
}

fn main() {
    let _ = hoge::bar();
    let _ = hoge::failable(123);
    let _ = hoge::fuga();
}
