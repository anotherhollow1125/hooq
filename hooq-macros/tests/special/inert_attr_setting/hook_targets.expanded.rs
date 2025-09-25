use hooq_macros::hooq;
fn hoge() -> Result<(), ()> {
    Ok(())
}
fn default(flag: bool) -> Result<(), ()> {
    hoge()
        .inspect_err(|e| {
            let path = "<hooq_root>/tests/special/inert_attr_setting/hook_targets.rs";
            let line = 10usize;
            {
                ::std::io::_eprint(format_args!("[{0}:L{1}] {2:?}\n", path, line, e));
            };
        })?;
    if !flag {
        return Err(())
            .inspect_err(|e| {
                let path = "<hooq_root>/tests/special/inert_attr_setting/hook_targets.rs";
                let line = 13usize;
                {
                    ::std::io::_eprint(
                        format_args!("[{0}:L{1}] {2:?}\n", path, line, e),
                    );
                };
            });
    }
    Err(())
        .inspect_err(|e| {
            let path = "<hooq_root>/tests/special/inert_attr_setting/hook_targets.rs";
            let line = 16usize;
            {
                ::std::io::_eprint(format_args!("[{0}:L{1}] {2:?}\n", path, line, e));
            };
        })
}
fn q_only(flag: bool) -> Result<(), ()> {
    hoge()
        .inspect_err(|e| {
            let path = "<hooq_root>/tests/special/inert_attr_setting/hook_targets.rs";
            let line = 22usize;
            {
                ::std::io::_eprint(format_args!("[{0}:L{1}] {2:?}\n", path, line, e));
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
                {
                    ::std::io::_eprint(
                        format_args!("[{0}:L{1}] {2:?}\n", path, line, e),
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
            {
                ::std::io::_eprint(format_args!("[{0}:L{1}] {2:?}\n", path, line, e));
            };
        })
}
fn return_and_question(flag: bool) -> Result<(), ()> {
    hoge()
        .inspect_err(|e| {
            let path = "<hooq_root>/tests/special/inert_attr_setting/hook_targets.rs";
            let line = 58usize;
            {
                ::std::io::_eprint(format_args!("[{0}:L{1}] {2:?}\n", path, line, e));
            };
        })?;
    if !flag {
        return Err(())
            .inspect_err(|e| {
                let path = "<hooq_root>/tests/special/inert_attr_setting/hook_targets.rs";
                let line = 61usize;
                {
                    ::std::io::_eprint(
                        format_args!("[{0}:L{1}] {2:?}\n", path, line, e),
                    );
                };
            });
    }
    Err(())
}
