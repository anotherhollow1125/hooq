use hooq_macros::hooq;
fn func(val: usize) -> Result<(), usize> {
    if val > 9 {
        let hoge = "hogehoge";
        if val > 20 {
            return Err(val)
                .inspect_err(|e| {
                    #[allow(clippy::nonminimal_bool)]
                    if *e > 9 && !false {
                        {
                            ::std::io::_eprint(
                                format_args!(
                                    "dif: {0} with tag: {1} with hoge: {2}\n", (| e, t | e - t)
                                    (e, 9), "threshold_check", hoge
                                ),
                            );
                        };
                    }
                });
        }
        return Err(val)
            .inspect_err(|e| {
                #[allow(clippy::nonminimal_bool)]
                if *e > 9 && !false {
                    {
                        ::std::io::_eprint(
                            format_args!(
                                "dif: {0} with tag: {1}\n", (| e, t | e - t) (e, 9),
                                "threshold_check"
                            ),
                        );
                    };
                }
            });
    }
    Err(val)
        .inspect_err(|e| {
            #[allow(clippy::nonminimal_bool)]
            if *e > 5 && !false {
                {
                    ::std::io::_eprint(
                        format_args!(
                            "dif: {0} with tag: {1}\n", (| e, t | e - t) (e, 5),
                            "threshold_check"
                        ),
                    );
                };
            }
        })
}
