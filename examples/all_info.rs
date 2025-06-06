#![allow(deprecated)]

use clap::Parser;
use hooq::hooq;

#[hooq]
#[hooq::method(.map(|v| {
    println!(
"$line: {}
$flle: {}
$expr: {}
$nth: {} ($count: {})
$tag: {}
$fn_name: {}
$fn_sig: {}
",
    $line,
    $file,
    $expr,
    $nth,
    $count,
    $tag,
    $fn_name,
    $fn_sig,
    );
    v
}).map_err(|e| {
    println!(
"$line: {}
$flle: {}
$expr: {}
$nth: {} ($count: {})
$tag: {}
$fn_name: {}
$fn_sig: {}
",
    $line,
    $file,
    $expr,
    $nth,
    $count,
    $tag,
    $fn_name,
    $fn_sig,
    );
    e
}))]
fn func(n: usize) -> Result<&'static str, &'static str> {
    println!("called func({})\n", n);

    fn check_four(n: usize) -> Result<(), &'static str> {
        if n == 4 {
            return Err("4 is unlucky number!");
        }
        Ok(())
    }

    check_four(n)?;

    if n % 2 == 0 {
        return Ok("even");
    }

    Ok("odd")
}

#[derive(Parser)]
struct Input {
    #[clap(short, long, default_value_t = 3)]
    n: usize,
}

fn main() {
    let Input { n } = Input::parse();

    func(n).unwrap();
}
