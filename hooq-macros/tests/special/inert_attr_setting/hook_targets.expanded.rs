use hooq_macros::hooq;
fn hoge() -> Result<(), ()> {
    Ok(())
}
fn default(flag: bool) -> Result<(), ()> {
    hoge()
        .inspect_err(|e| {
            let path = "<hooq_root>/tests/special/inert_attr_setting/hook_targets.rs";
            let line = 10usize;
            let col = 11usize;
            let expr = "  10|     hoge()?\n    |";
            {
                ::std::io::_eprint(
                    format_args!("[{0}:{1}:{2}] {3:?}\n{4}\n", path, line, col, e, expr),
                );
            };
        })?;
    if !flag {
        return Err(())
            .inspect_err(|e| {
                let path = "<hooq_root>/tests/special/inert_attr_setting/hook_targets.rs";
                let line = 13usize;
                let col = 9usize;
                let expr = "  13|         return Err(())\n    |";
                {
                    ::std::io::_eprint(
                        format_args!(
                            "[{0}:{1}:{2}] {3:?}\n{4}\n", path, line, col, e, expr
                        ),
                    );
                };
            });
    }
    Err(())
        .inspect_err(|e| {
            let path = "<hooq_root>/tests/special/inert_attr_setting/hook_targets.rs";
            let line = 16usize;
            let col = 5usize;
            let expr = "  16|     Err(())\n    |";
            {
                ::std::io::_eprint(
                    format_args!("[{0}:{1}:{2}] {3:?}\n{4}\n", path, line, col, e, expr),
                );
            };
        })
}
fn q_only(flag: bool) -> Result<(), ()> {
    hoge()
        .inspect_err(|e| {
            let path = "<hooq_root>/tests/special/inert_attr_setting/hook_targets.rs";
            let line = 22usize;
            let col = 11usize;
            let expr = "  22|     hoge()?\n    |";
            {
                ::std::io::_eprint(
                    format_args!("[{0}:{1}:{2}] {3:?}\n{4}\n", path, line, col, e, expr),
                );
            };
        })?;
    if !flag {
        return Err(());
    }
    Err(())
}
fn return_only(flag: bool) -> Result<(), ()> {
    hoge()?;
    if !flag {
        return Err(())
            .inspect_err(|e| {
                let path = "<hooq_root>/tests/special/inert_attr_setting/hook_targets.rs";
                let line = 37usize;
                let col = 9usize;
                let expr = "  37|         return Err(())\n    |";
                {
                    ::std::io::_eprint(
                        format_args!(
                            "[{0}:{1}:{2}] {3:?}\n{4}\n", path, line, col, e, expr
                        ),
                    );
                };
            });
    }
    Err(())
}
fn tailexpr_only(flag: bool) -> Result<(), ()> {
    hoge()?;
    if !flag {
        return Err(());
    }
    Err(())
        .inspect_err(|e| {
            let path = "<hooq_root>/tests/special/inert_attr_setting/hook_targets.rs";
            let line = 52usize;
            let col = 5usize;
            let expr = "  52|     Err(())\n    |";
            {
                ::std::io::_eprint(
                    format_args!("[{0}:{1}:{2}] {3:?}\n{4}\n", path, line, col, e, expr),
                );
            };
        })
}
fn return_and_question(flag: bool) -> Result<(), ()> {
    hoge()
        .inspect_err(|e| {
            let path = "<hooq_root>/tests/special/inert_attr_setting/hook_targets.rs";
            let line = 58usize;
            let col = 11usize;
            let expr = "  58|     hoge()?\n    |";
            {
                ::std::io::_eprint(
                    format_args!("[{0}:{1}:{2}] {3:?}\n{4}\n", path, line, col, e, expr),
                );
            };
        })?;
    if !flag {
        return Err(())
            .inspect_err(|e| {
                let path = "<hooq_root>/tests/special/inert_attr_setting/hook_targets.rs";
                let line = 61usize;
                let col = 9usize;
                let expr = "  61|         return Err(())\n    |";
                {
                    ::std::io::_eprint(
                        format_args!(
                            "[{0}:{1}:{2}] {3:?}\n{4}\n", path, line, col, e, expr
                        ),
                    );
                };
            });
    }
    Err(())
}
