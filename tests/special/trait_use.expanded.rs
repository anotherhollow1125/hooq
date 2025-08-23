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
    use hooq::hooq;
    use super::trait_define::Hook1 as _;
    use super::trait_define::Hook2 as _;
    pub fn use_hook<T, E>(result: Result<T, E>) -> Result<T, E>
    where
        E: std::fmt::Debug,
    {
        result
            .hook1()
            .hook2()
            .inspect_err(|e| {
                let path = "<hooq_root>/tests/special/trait_use.rs";
                let line = 32usize;
                {
                    ::std::io::_eprint(
                        format_args!("{0:?} @ path: {1}, line: {2}\n", e, path, line),
                    );
                };
            })
    }
}
mod custom {
    use hooq::hooq;
    use super::trait_define::Hook2 as _;
    pub fn use_hook<T, E>(result: Result<T, E>) -> Result<T, E>
    where
        E: std::fmt::Debug,
    {
        result
            .hook2()
            .inspect_err(|e| {
                let path = "<hooq_root>/tests/special/trait_use.rs";
                let line = 44usize;
                {
                    ::std::io::_eprint(
                        format_args!("{0:?} @ path: {1}, line: {2}\n", e, path, line),
                    );
                };
            })
    }
}
