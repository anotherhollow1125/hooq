use hooq_macros::hooq;
mod tmp {
    #[allow(unused)]
    fn hoge() -> Result<(), ()> {
        Ok(())
            .inspect(|_| {
                {
                    ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
                };
            })
    }
    pub const STATIC_VAR: u32 = {
        fn _f(flag: bool) -> Result<(), ()> {
            if flag {
                return Err(())
                    .inspect(|_| {
                        {
                            ::std::io::_print(format_args!("tag: {0}\n", "inner func"));
                        };
                    });
            }
            hoge()
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0}\n", "inner func"));
                    };
                })?;
            let res = {
                if flag {
                    return hoge()
                        .inspect(|_| {
                            {
                                ::std::io::_print(format_args!("tag: {0}\n", "deep"));
                            };
                        });
                }
                Ok(())
                    .inspect(|_| {
                        {
                            ::std::io::_print(format_args!("tag: {0}\n", "deep"));
                        };
                    })
            };
            res.inspect(|_| {
                {
                    ::std::io::_print(format_args!("tag: {0}\n", "inner func"));
                };
            })
        }
        10
    };
}
