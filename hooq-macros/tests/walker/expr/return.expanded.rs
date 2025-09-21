use hooq_macros::hooq;
fn func(flag: bool) -> Result<(), ()> {
    if flag {
        return Err(())
            .inspect(|_| {
                {
                    ::std::io::_print(format_args!("tag: {0}\n", "return"));
                };
            });
    }
    fn hoge(flag: bool) -> u32 {
        if flag {
            return 1;
        }
        0
    }
    let _ = hoge(flag);
    Err(())
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
            };
        })
}
