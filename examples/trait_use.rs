use hooq::hooq;

mod sub_mod {
    use std::fmt::Debug;

    pub trait Hook {
        fn hook(self, message: &str) -> Self;
    }

    impl<T, E> Hook for Result<T, E>
    where
        E: Debug,
    {
        fn hook(self, message: &str) -> Self {
            if let Err(e) = &self {
                eprintln!("Error occurred: {e:?} with message {message}");
            }

            self
        }
    }
}

#[hooq(trait_use(sub_mod::Hook))]
#[hooq::method(.hook(format!("[Error Occurred @ line {}]", $line).as_str()))]
fn check_name(name: String) -> Result<String, String> {
    if name == "Voldemort" {
        return Err("This name is not allowed.".into());
    }

    Ok(name)
}

#[hooq]
#[hooq::method(.hook(format!("[Error Occurred @ line {}]", $line).as_str()))]
fn main() -> Result<(), ()> {
    let name = String::from("Alice");
    let num = 3;

    let name = check_name(name).map_err(|_| ())?;

    if num == 4 {
        return Err(());
    }

    for _ in 0..num {
        println!("Hello, {name}!");
    }

    Ok(())
}
