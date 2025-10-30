use hooq_macros::hooq;
fn hoge() -> Result<(), ()> {
    Err(())
        .inspect_err(|e| {
            let path = "<hooq_root>/tests/walker/expr/break.rs";
            let line = 5usize;
            let col = 5usize;
            let expr = "Err(())";
            {
                ::std::io::_eprint(
                    format_args!(
                        "[{0}:{1}:{2}] {3:?}\n    {4}\n", path, line, col, e, expr
                    ),
                );
            };
        })
}
fn func(flag: bool) -> Result<(), ()> {
    let mut counter = 0;
    loop {
        counter += 1;
        if counter > 2 {
            break;
        }
    }
    loop {
        counter += 1;
        if counter > 3 {
            break Err(());
        }
    }
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
            };
        })?;
    'outer: loop {
        counter += 1;
        if counter > 4 {
            loop {
                counter += 1;
                if counter > 5 {
                    break 'outer;
                }
            }
        }
    }
    loop {
        counter += 1;
        if counter > 10 {
            break {
                {
                    hoge()
                        .inspect(|_| {
                            {
                                ::std::io::_print(format_args!("tag: {0}\n", "nest"));
                            };
                        })?;
                    if !flag {
                        return Err(())
                            .inspect(|_| {
                                {
                                    ::std::io::_print(format_args!("tag: {0}\n", "return"));
                                };
                            });
                    }
                    Err(())
                        .inspect(|_| {
                            {
                                ::std::io::_print(format_args!("tag: {0}\n", "nest"));
                            };
                        })
                }
            };
        }
    }
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0}\n", "check nest"));
            };
        })
}
