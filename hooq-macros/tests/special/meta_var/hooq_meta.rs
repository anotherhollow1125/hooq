use hooq_macros::hooq;

mod trait_define {
    use std::fmt::Debug;

    pub trait CustomHook {
        fn hook(self, f: impl FnOnce() -> hooq::HooqMeta) -> Self;
    }

    impl<T, E> CustomHook for Result<T, E>
    where
        T: Debug,
        E: Debug,
    {
        fn hook(self, f: impl FnOnce() -> hooq::HooqMeta) -> Self {
            let info = f();
            eprintln!("{info:?}");

            if let Some(detail) = info.get_binding::<bool>("detail")
                && *detail
            {
                println!("detail: {self:?}");
            }

            self
        }
    }
}

mod custom {
    use hooq_macros::hooq;

    #[hooq(hook, trait_use(super::trait_define::CustomHook))]
    #[hooq::tail_expr_idents("Ok", "Err")]
    #[hooq::not_tail_expr_idents()]
    #[hooq::hoge = "hogehoge"]
    #[hooq::inner_struct = InnerStruct]
    #[hooq::array = [1, 2, 3]]
    pub fn use_hook(result: Result<i32, ()>) -> Result<i32, ()> {
        #[hooq::detail = true]
        if let Ok(val) = result
            && val >= 100
        {
            return Ok(100);
        }

        #[derive(Debug)]
        struct InnerStruct;

        #[hooq::tuple = (InnerStruct, 10)]
        #[hooq::fuga = 10]
        #[hooq::bool_val = true]
        result
    }

    #[hooq(trait_use(super::trait_define::CustomHook), hook)]
    #[hooq::tail_expr_idents("Ok", "Err")]
    pub fn use_hook2() -> Result<i32, ()> {
        Ok(42)
    }

    #[hooq(trait_use(super::trait_define::CustomHook), flavor = "hook")]
    #[hooq::tail_expr_idents("Ok", "Err")]
    pub fn use_hook3() -> Result<i32, ()> {
        Ok(42)
    }
}

#[hooq]
#[hooq::method(.inspect(|_| {
    let meta = $hooq_meta;
    println!("{meta:?}");

    if let Some(fuga) = meta.get_binding::<&'static str>("fuga") {
        println!("fuga: {fuga}");
    }
}))]
#[hooq::var(hoge = "hoge")]
#[hooq::tail_expr_idents("Ok", "Err")]
fn using_hooq_meta(flag: bool) -> Result<(), ()> {
    if flag {
        Ok(())
    } else {
        #[hooq::fuga = "fugafuga"]
        Ok(())
    }
}

#[test]
fn test() {
    custom::use_hook(Ok(42)).unwrap();
    custom::use_hook(Ok(150)).unwrap();
    custom::use_hook2().unwrap();
    custom::use_hook3().unwrap();
    using_hooq_meta(true).unwrap();
    using_hooq_meta(false).unwrap();
}
