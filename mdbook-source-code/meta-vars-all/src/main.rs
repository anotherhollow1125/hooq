use hooq::hooq;

fn fallible<T>(val: T) -> Result<T, String> {
    Ok(val)
}

#[hooq]
#[hooq::xxx = "user defined binding."]
#[hooq::method(.inspect_err(|_| {
    // Fundamental information provided by hooq.
    let _line = $line;
    let _column = $column;
    let _path = $path;
    let _file = $file;
    let _source = stringify!($source);
    let _count = $count;
    let _fn_name = $fn_name;
    let _fn_sig = $fn_sig;

    // Meta vars defined by user.
    let _xxx = $xxx;
    let _bindings = $bindings;

    // All information summarized up to this point.
    let _hooq_meta = $hooq_meta;
}))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    fallible(())?;

    Ok(())
}
