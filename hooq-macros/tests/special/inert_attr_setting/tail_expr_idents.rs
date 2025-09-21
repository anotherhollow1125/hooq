use hooq_macros::hooq;

// not_tail_expr_idents も参照されたし
//
// なんだかんだ result_types の方にも
// tail_expr_idents が関わる
// テストを書いたのでそちらも参照されたし

fn enresult<T>(val: T) -> Result<T, ()> {
    Ok(val)
}

#[hooq]
#[hooq::result_types()]
#[hooq::tail_expr_idents("Right", "Left")]
#[hooq::not_tail_expr_idents()]
fn func() -> Result<(), ()> {
    #[allow(non_snake_case)]
    let Right = || Result::<(), ()>::Ok(());
    #[allow(non_snake_case)]
    let Left = || Result::<(), ()>::Err(());

    // ? へのフックは相変わらず
    enresult(())?;

    // Ok や Err にはフックしない
    if enresult(false)? {
        return Ok(());
    }

    if enresult(false)? {
        return Err(());
    }

    // 指定したものにはフックする
    if enresult(false)? {
        return Left();
    }

    let _ = || Right();

    Right()
}

#[test]
fn test() {
    func().unwrap();
}
