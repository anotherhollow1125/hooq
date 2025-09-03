use hooq_macros::hooq;
fn hoge() -> Result<(), ()> {
    Ok(())
        .inspect_err(|e| {
            let path = "<hooq_root>/tests/special/hook_targets.rs";
            let line = 5usize;
            {
                ::std::io::_eprint(format_args!("[{0}:L{1}] {2:?}\n", path, line, e));
            };
        })
}
fn default(flag: bool) -> Result<(), ()> {
    hoge()
        .inspect_err(|e| {
            let path = "<hooq_root>/tests/special/hook_targets.rs";
            let line = 10usize;
            {
                ::std::io::_eprint(format_args!("[{0}:L{1}] {2:?}\n", path, line, e));
            };
        })?;
    if !flag {
        return Ok(())
            .inspect_err(|e| {
                let path = "<hooq_root>/tests/special/hook_targets.rs";
                let line = 13usize;
                {
                    ::std::io::_eprint(
                        format_args!("[{0}:L{1}] {2:?}\n", path, line, e),
                    );
                };
            });
    }
    Ok(())
        .inspect_err(|e| {
            let path = "<hooq_root>/tests/special/hook_targets.rs";
            let line = 16usize;
            {
                ::std::io::_eprint(format_args!("[{0}:L{1}] {2:?}\n", path, line, e));
            };
        })
}
fn q_only(flag: bool) -> Result<(), ()> {
    hoge()
        .inspect_err(|e| {
            let path = "<hooq_root>/tests/special/hook_targets.rs";
            let line = 22usize;
            {
                ::std::io::_eprint(format_args!("[{0}:L{1}] {2:?}\n", path, line, e));
            };
        })?;
    if !flag {
        return Ok(());
    }
    Ok(())
}
fn return_only(flag: bool) -> Result<(), ()> {
    hoge()?;
    if !flag {
        return Ok(())
            .inspect_err(|e| {
                let path = "<hooq_root>/tests/special/hook_targets.rs";
                let line = 37usize;
                {
                    ::std::io::_eprint(
                        format_args!("[{0}:L{1}] {2:?}\n", path, line, e),
                    );
                };
            });
    }
    Ok(())
}
fn tailexpr_only(flag: bool) -> Result<(), ()> {
    hoge()?;
    if !flag {
        return Ok(());
    }
    Ok(())
        .inspect_err(|e| {
            let path = "<hooq_root>/tests/special/hook_targets.rs";
            let line = 52usize;
            {
                ::std::io::_eprint(format_args!("[{0}:L{1}] {2:?}\n", path, line, e));
            };
        })
}
fn return_and_question(flag: bool) -> Result<(), ()> {
    hoge()
        .inspect_err(|e| {
            let path = "<hooq_root>/tests/special/hook_targets.rs";
            let line = 58usize;
            {
                ::std::io::_eprint(format_args!("[{0}:L{1}] {2:?}\n", path, line, e));
            };
        })?;
    if !flag {
        return Ok(())
            .inspect_err(|e| {
                let path = "<hooq_root>/tests/special/hook_targets.rs";
                let line = 61usize;
                {
                    ::std::io::_eprint(
                        format_args!("[{0}:L{1}] {2:?}\n", path, line, e),
                    );
                };
            });
    }
    Ok(())
}
