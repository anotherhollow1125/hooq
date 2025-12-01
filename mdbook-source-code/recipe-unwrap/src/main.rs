use hooq::hooq;

fn failable<T>(val: T) -> Result<T, String> {
    Ok(val)
}

#[cfg_attr(not(feature = "unwrap"), hooq(empty))]
#[cfg_attr(feature = "unwrap", hooq(unwrap))]
fn process(flag: bool) -> Result<(), String> {
    if flag {
        return Err("An error occurred".into());
    }

    let _ = failable(42)?;

    Ok(())
}

#[cfg_attr(not(feature = "unwrap"), hooq(empty))]
#[cfg_attr(feature = "unwrap", hooq(unwrap))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    process(false)?;

    Ok(())
}
