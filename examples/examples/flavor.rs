use hooq::hooq;

#[hooq(custom)]
fn func(n: i32) -> Result<(), String> {
    if n < 0 {
        return Err("negative number".to_string());
    }

    if n == 0 {
        return Ok(());
    }

    println!("n: {}", n);

    func(n - 1)?;

    #[hooq::method = custom2]
    if n % 2 == 0 {
        println!("even: {}", n);

        func(n / 2)?;
    }

    Ok(())
}

#[hooq(flavor = "custom")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    func(42)?;

    Ok(())
}
