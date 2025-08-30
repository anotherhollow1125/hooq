use hooq_macros::hooq;
fn func(val: usize) -> Result<(), usize> {
    Err(val)
        .inspect_err(|e| {
            if *e > 5 {
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
