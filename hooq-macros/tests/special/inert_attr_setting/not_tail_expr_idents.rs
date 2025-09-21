use hooq_macros::hooq;

fn enresult<T>(val: T) -> Result<T, ()> {
    Ok(val)
}

#[hooq]
// Result 型を対象にしていても、not_tail_expr_idents に含まれる場合はフックされないことを確認する
#[hooq::result_types("Result")]
#[hooq::tail_expr_idents("Right")] // 両方含めるものは tail_expr_idents のテストで検証済み
#[hooq::not_tail_expr_idents("Left")]
fn func() -> Result<(), ()> {
    #[allow(non_snake_case)]
    let Right = || Result::<(), ()>::Ok(());
    #[allow(non_snake_case)]
    let Left = || Result::<(), ()>::Err(());

    // ? へのフックは相変わらず
    enresult(())?;

    // 指定したものにはフックする
    if enresult(false)? {
        return Right();
    }

    let _ = || Right();

    // return_types 指定に反しフックしない！
    if enresult(false)? {
        return Left();
    }

    // return_types 指定のため以下はフックされる
    if enresult(false)? {
        return Ok(());
    }

    if enresult(false)? {
        return Err(());
    }

    if enresult(false)? {
        return enresult(());
    }

    // Result型が返り値なのでフックされるはずだが、しない！
    Left()
}

// "!Xxx" 記法がしっかり機能していることを確かめる
#[hooq]
#[hooq::result_types("Result")]
#[hooq::tail_expr_idents("Right", "!Left")]
fn func2() -> Result<(), ()> {
    #[allow(non_snake_case)]
    let Right = || Result::<(), ()>::Ok(());
    #[allow(non_snake_case)]
    let Left = || Result::<(), ()>::Err(());

    enresult(())?;

    if enresult(false)? {
        return Right();
    }

    let _ = || Right();

    if enresult(false)? {
        return Left();
    }

    if enresult(false)? {
        return Ok(());
    }

    if enresult(false)? {
        return Err(());
    }

    if enresult(false)? {
        return enresult(());
    }

    Left()
}

#[test]
fn test() {
    func().unwrap_err();
    func2().unwrap_err();
}
