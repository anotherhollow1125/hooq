use hooq::hooq;
use tracing::instrument;

#[hooq(tracing)]
#[instrument]
fn func1() -> Result<i32, String> {
    Err("Error in func1".into())
}

#[hooq(tracing)]
#[instrument]
fn func2() -> Result<i32, String> {
    println!("func2 start");

    let res = func1()?;

    println!("func2 end: {res}");

    Ok(res)
}

#[hooq(tracing)]
#[instrument]
fn func3() -> Result<i32, String> {
    println!("func3 start");

    let res = func2()?;

    println!("func3 end: {res}");

    Ok(res)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let format = tracing_subscriber::fmt::format()
        .with_ansi(false)
        .without_time();
    tracing_subscriber::fmt().event_format(format).init();

    func3()?;

    Ok(())
}
