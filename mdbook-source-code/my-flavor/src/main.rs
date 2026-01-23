use hooq::hooq;

fn fallible<T>(val: T) -> Result<T, String> {
    Ok(val)
}

mod sub {
    pub trait MyTrait {}
}

#[hooq(my_flavor)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    fallible(())?;

    #[hooq::flavor = my_flavor::sub_flavor]
    fallible(())?;

    Ok(())
}
