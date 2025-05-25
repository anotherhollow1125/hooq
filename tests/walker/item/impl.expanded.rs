use hooq::hooq;
#[cfg(all())]
fn func() -> Result<(), ()> {
    struct S;
    #[cfg(all())]
    impl S {
        #[hooq::tag("impl related function")]
        fn g() -> Result<(), ()> {
            Ok(())
                .map(|v| {
                    {
                        ::std::io::_print(format_args!("tag: {0:?}\n", "impl"));
                    };
                    v
                })
        }
        #[hooq::tag("impl related function 2 (not Result)")]
        fn h() -> bool {
            true
        }
        #[hooq::tag("impl method")]
        fn i(&self) -> Result<(), ()> {
            let res = Ok(());
            res.map(|v| {
                {
                    ::std::io::_print(format_args!("tag: {0:?}\n", "impl"));
                };
                v
            })
        }
        #[hooq::tag("impl method 2 (not Result)")]
        fn j(&self) -> bool {
            true
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
