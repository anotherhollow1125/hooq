use hooq::hooq;

#[hooq]
#[hooq::method(.inspect_err(|_| {
    let source = ::hooq::summary!($source);

    eprintln!("{source}");
}))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    Err((
        "aaaaaaaaaaaaaaaaaaaa",
        "bbbbbbbbbbbbbbbbbbbb",
        "cccccccccccccccccccc",
        "dddddddddddddddddddd",
        "errorerrorerrorerrorerror",
    )
        .4
        .into())
}
