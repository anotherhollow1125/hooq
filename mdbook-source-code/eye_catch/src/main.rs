use hooq::hooq;

#[hooq]
#[hooq::method(.map(|v| v * 2))]
fn double(s: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let res = s.parse::<u32>()?;
    Ok(res)
}

fn double_expanded(s: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let res = s.parse::<u32>().map(|v| v * 2)?;
    Ok(res)
}

#[test]
fn test() {
    assert_eq!(double("21").unwrap(), double_expanded("21").unwrap());
}

fn main() {
    println!("double_hooked: {}", double("21").unwrap());
    println!("double: {}", double_expanded("21").unwrap());
}
