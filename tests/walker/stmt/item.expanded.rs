use hooq::hooq;
#[cfg(all())]
fn func() -> Result<(), ()> {
    #[cfg(all())]
    fn f() -> Result<(), ()> {
        Ok(())
            .map(|v| {
                {
                    ::std::io::_print(format_args!("tag: {0:?}\n", "func"));
                };
                v
            })
    }
    f()
        .map(|v| {
            {
                ::std::io::_print(format_args!("tag: {0:?}\n", "(no tag)"));
            };
            v
        })?;
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
