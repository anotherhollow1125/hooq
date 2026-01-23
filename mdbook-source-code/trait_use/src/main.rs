use hooq::hooq;

mod sub {
    pub trait Inserted {
        fn inserted(self) -> Self;
    }

    impl<T, E> Inserted for Result<T, E> {
        fn inserted(self) -> Self {
            self
        }
    }
}

fn fallible<T>(val: T) -> Result<T, String> {
    Ok(val)
}

#[hooq(trait_use(sub::Inserted))]
#[hooq::method(.inserted())]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    fallible(())?;

    Ok(())
}
