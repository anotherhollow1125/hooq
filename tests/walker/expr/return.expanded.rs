use hooq::hooq;
#[cfg(all())]
fn func(flag: bool) -> Result<(), ()> {
    #[cfg(all())]
    if flag {
        return Ok(())
            .map(|v| {
                {
                    ::std::io::_print(format_args!("tag: {0:?}\n", "return"));
                };
                v
            });
    }
    #[cfg(all())]
    fn hoge(flag: bool) -> u32 {
        if flag {
            return 1
                .map(|v| {
                    {
                        ::std::io::_print(format_args!("tag: {0:?}\n", "not result"));
                    };
                    v
                });
        }
        0
    }
    Ok(())
        .map(|v| {
            {
                ::std::io::_print(format_args!("tag: {0:?}\n", "(no tag)"));
            };
            v
        })
}
