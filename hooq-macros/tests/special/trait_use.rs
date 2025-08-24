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
}

mod trait_use_inner {
    use hooq_macros::hooq;

    #[hooq(trait_use(super::trait_define::Hook1, super::trait_define::Hook2))]
    pub fn use_hook<T, E>(result: Result<T, E>) -> Result<T, E>
    where
        E: std::fmt::Debug,
    {
        result.hook1().hook2()
    }
}

mod custom {
    use hooq_macros::hooq;

    #[hooq(custom(super::trait_define::Hook2))]
    pub fn use_hook<T, E>(result: Result<T, E>) -> Result<T, E>
    where
        E: std::fmt::Debug,
    {
        result.hook2()
    }
}

#[test]
fn test() {
    let result: Result<i32, &str> = Ok(42);
    trait_use_inner::use_hook(result).unwrap();
    custom::use_hook(result).unwrap();
}
