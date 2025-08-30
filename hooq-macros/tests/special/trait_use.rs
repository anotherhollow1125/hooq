mod trait_define {

    pub trait Hook1 {
        fn hook1(self) -> Self;
    }

    impl<T, E> Hook1 for Result<T, E> {
        fn hook1(self) -> Self {
            self
        }
    }

    pub trait Hook2 {
        fn hook2(self) -> Self;
    }

    impl<T, E> Hook2 for Result<T, E> {
        fn hook2(self) -> Self {
            self
        }
    }

    pub trait CustomHook {
        fn hook(self, f: impl FnOnce() -> hooq::HooqMeta) -> Self;
    }

    impl<T, E> CustomHook for Result<T, E> {
        fn hook(self, f: impl FnOnce() -> hooq::HooqMeta) -> Self {
            let info = f();
            eprintln!("{info:?}");
            self
        }
    }
}

mod trait_use_inner {
    use hooq_macros::hooq;

    #[hooq(trait_use(super::trait_define::Hook1, super::trait_define::Hook2))]
    #[hooq::method(.hook2())]
    pub fn use_hook<T, E>(result: Result<T, E>) -> Result<T, E>
    where
        E: std::fmt::Debug,
    {
        result.hook1()
    }
}

mod custom {
    use hooq_macros::hooq;

    #[hooq(custom(super::trait_define::CustomHook))]
    pub fn use_hook<T, E>(result: Result<T, E>) -> Result<T, E>
    where
        E: std::fmt::Debug,
    {
        result
    }
}

mod custom2 {
    use hooq_macros::hooq;

    use super::trait_define::CustomHook;

    #[hooq(custom)]
    pub fn use_hook<T, E>(result: Result<T, E>) -> Result<T, E>
    where
        E: std::fmt::Debug,
    {
        result
    }

    #[hooq(custom = true)]
    pub fn use_hook2<T, E>(result: Result<T, E>) -> Result<T, E>
    where
        E: std::fmt::Debug,
    {
        result
    }

    #[hooq(preset = "custom")]
    pub fn use_hook3<T, E>(result: Result<T, E>) -> Result<T, E>
    where
        E: std::fmt::Debug,
    {
        result
    }
}

#[test]
fn test() {
    let result: Result<i32, &str> = Ok(42);
    trait_use_inner::use_hook(result).unwrap();
    custom::use_hook(result).unwrap();
    custom2::use_hook(result).unwrap();
    custom2::use_hook2(result).unwrap();
    custom2::use_hook3(result).unwrap();
}
