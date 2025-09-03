use hooq_macros::hooq;

// NOTE:
// tail_expr_idents および
// result_types
// の両方を同時に検証するテスト
//
// Option 型向けにも使えることを示すことで
// 両機能を検証する

#[hooq]
fn hoge() -> Result<(), ()> {
    Ok(())
}

#[hooq]
#[hooq::result_types("Option")]
#[hooq::tail_expr_idents("Some", "None")]
#[hooq::method(.inspect(|_| {
    eprintln!("{:?}", $hooq_meta);
}))]
fn option_fn_1(flag: bool, flagflag: bool) -> Option<()> {
    hoge().ok()?;

    if !flag {
        return None;
    }

    let _ = || {
        // Option かわからない状況
        if flag {
            return Some(());
        }

        if flagflag {
            return None;
        }

        None
    };

    Some(())
}

#[hooq]
#[hooq::result_types("Option")]
#[hooq::tail_expr_idents("Some")]
#[hooq::method(.inspect(|_| {
    eprintln!("{:?}", $hooq_meta);
}))]
fn option_fn_2(flag: bool, flagflag: bool) -> Option<()> {
    hoge().ok()?;

    if !flag {
        // Option型なのでフックされてしまう
        return None;
    }

    let _ = || {
        // Option かわからない状況
        if flag {
            return Some(());
        }

        if flagflag {
            return None;
        }

        None
    };

    None
}

#[test]
fn test() {
    option_fn_1(true, false).unwrap();
    option_fn_2(true, false).unwrap_or_default();
}
