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
        fn hook(self, f: impl FnOnce() -> hooq::HooqInfo) -> Self;
    }
    impl<T, E> CustomHook for Result<T, E> {
        fn hook(self, f: impl FnOnce() -> hooq::HooqInfo) -> Self {
            let info = f();
            {
                ::std::io::_eprint(format_args!("{0:?}\n", info));
            };
            self
        }
    }
}
mod trait_use_inner {
    use hooq_macros::hooq;
    use super::trait_define::Hook1 as _;
    use super::trait_define::Hook2 as _;
    pub fn use_hook<T, E>(result: Result<T, E>) -> Result<T, E>
    where
        E: std::fmt::Debug,
    {
        result.hook1().hook2()
    }
}
mod custom {
    use hooq_macros::hooq;
    use super::trait_define::CustomHook as _;
    pub fn use_hook<T, E>(result: Result<T, E>) -> Result<T, E>
    where
        E: std::fmt::Debug,
    {
        result
            .hook(|| {
                ::hooq::HooqInfo {
                    line: 57usize,
                    column: 9usize,
                    path: "<hooq_root>/tests/special/trait_use.rs",
                    abs_path: "<hooq_root>/tests/special/trait_use.rs",
                    file: "trait_use.rs",
                    expr: "result",
                    count: "1th tail expr",
                }
            })
    }
}
