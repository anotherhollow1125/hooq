use hooq::hooq;

fn failable<T>(val: T) -> Result<T, String> {
    Ok(val)
}

type MyResult = Result<(), String>;

#[hooq]
#[hooq::method(.inspect_err(|_| {}))]
fn func1() -> MyResult {
    let _ = || -> Result<(), String> { failable(()) };

    failable(())
}

#[hooq]
#[hooq::method(.inspect_err(|_| {}))]
#[hooq::result_types("MyResult")]
fn func2() -> MyResult {
    // No longer hooked.
    let _ = || -> Result<(), String> { failable(()) };

    failable(())
}

#[hooq]
#[hooq::method(.inspect_err(|_| {}))]
#[hooq::result_types("Result", "MyResult")]
fn func3() -> MyResult {
    let _ = || -> Result<(), String> { failable(()) };

    let _ = || {
        // Not hooked because return type of the closure is unknown.
        failable(())
    };

    failable(())
}

fn main() {
    let _ = func1();
    let _ = func2();
    let _ = func3();
}
