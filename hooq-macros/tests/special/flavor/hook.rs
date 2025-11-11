mod trait_define {
    pub trait CustomHook {
        fn hook(self, f: impl FnOnce() -> hooq::HooqMeta) -> Self;
    }

    impl<T, E> CustomHook for Result<T, E> {
        fn hook(self, f: impl FnOnce() -> hooq::HooqMeta) -> Self {
            let info = f();
            eprintln!("{info:?}");

            if let Some(val) = info.get_binding::<&str>("hoge") {
                eprintln!("hoge: {val}");
            }

            self
        }
    }
}

mod custom {
    use hooq::hooq;

    #[hooq(trait_use(super::trait_define::CustomHook), flavor = "hook")]
    #[hooq::hoge = "val"]
    pub fn use_hook<T, E>(result: Result<T, E>) -> Result<T, E>
    where
        E: std::fmt::Debug,
    {
        result
    }

    #[hooq(hook)]
    pub fn use_hook2<T, E>(result: Result<T, E>) -> Result<T, E>
    where
        E: std::fmt::Debug,
    {
        result
    }
}

mod custom2 {
    use hooq_macros::hooq;

    use super::trait_define::CustomHook;

    #[hooq(hook)]
    pub fn use_hook<T, E>(result: Result<T, E>) -> Result<T, E>
    where
        E: std::fmt::Debug,
    {
        result
    }

    #[hooq(flavor = "hook")]
    pub fn use_hook2<T, E>(result: Result<T, E>) -> Result<T, E>
    where
        E: std::fmt::Debug,
    {
        result
    }
}

#[test]
fn test() {
    let result: Result<i32, &str> = Ok(42);
    custom::use_hook(result).unwrap();
    custom::use_hook2(result).unwrap();
    custom2::use_hook(result).unwrap();
    custom2::use_hook2(result).unwrap();
}
