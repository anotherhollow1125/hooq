use hooq::hooq;

fn failable<T>(val: T) -> Result<T, String> {
    Ok(val)
}

enum CauseKind {
    DataBase,
    Server,
}

#[hooq]
// Can be defined in the format #[hooq::xxx = value]
#[hooq::string = "hello!"] // string literal
#[hooq::integer = 10] // integer literal
#[hooq::cause_kind = CauseKind::Server] // some value
#[hooq::method(.inspect_err(|_| {
    let _string = $string;
    let _integer = $integer;
    let _cause_kind = $cause_kind;
}))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    failable(())?;

    // Overriding meta variables.
    #[hooq::cause_kind = CauseKind::DataBase]
    failable(())?;

    Ok(())
}
