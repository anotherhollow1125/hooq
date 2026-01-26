use hooq::hooq;

mod my_error {
    pub trait MyHook {
        fn hook(self, meta_fn: impl FnOnce() -> hooq::HooqMeta) -> Self;
    }

    impl<T, E> MyHook for Result<T, E>
    where
        E: std::fmt::Debug,
    {
        fn hook(self, meta_fn: impl FnOnce() -> hooq::HooqMeta) -> Self {
            if let Err(e) = &self {
                let meta = meta_fn();

                eprintln!(
                    "[{}:{}:{}] error occurred: {:?}",
                    meta.file, meta.line, meta.column, e
                );
            }

            self
        }
    }
}

fn fallible<T>(val: T) -> Result<T, String> {
    Ok(val)
}

#[hooq(hook, trait_uses(my_error::MyHook))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    fallible(())?;

    Ok(())
}
