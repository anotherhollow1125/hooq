use hooq::hooq;
fn hoge() -> Result<(), ()> {
    Ok(())
        .inspect_err(|e| {
            {
                ::std::io::_eprint(
                    format_args!(
                        "{0:?} @ path: {1}, line: {2}\n", e,
                        "<hooq_root>/tests/walker/expr/break.rs", 5usize,
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
            break Ok(());
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
                    Ok(())
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
