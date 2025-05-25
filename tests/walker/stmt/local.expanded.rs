use hooq::hooq;
#[cfg(all())]
fn func() -> Result<(), ()> {
    let _ = #[cfg(all())]
    || {
        {
            #[cfg(all())]
            Ok(())
                .map(|v| {
                    {
                        ::std::io::_print(format_args!("tag: {0:?}\n", "inner"));
                    };
                    v
                })
        }
            .map(|v| {
                {
                    ::std::io::_print(format_args!("tag: {0:?}\n", "outer"));
                };
                v
            })?
    };
    let n = 1;
    #[cfg(all())]
    let 1 = n else {
        return Err(())
            .map(|v| {
                {
                    ::std::io::_print(format_args!("tag: {0:?}\n", "else"));
                };
                v
            });
    };
    Ok(())
        .map(|v| {
            {
                ::std::io::_print(format_args!("tag: {0:?}\n", "(no tag)"));
            };
            v
        })
}
fn main() {
    func().unwrap();
}
