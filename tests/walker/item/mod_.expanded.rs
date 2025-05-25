use hooq::hooq;
#[cfg(all())]
fn func() -> Result<(), ()> {
    #[cfg(all())]
    mod tmp {
        fn func() -> Result<(), ()> {
            Ok(())
                .map(|v| {
                    {
                        ::std::io::_print(format_args!("tag: {0:?}\n", "mod"));
                    };
                    v
                })
        }
    }
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
