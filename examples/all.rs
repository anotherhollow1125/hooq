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
}))]
fn func(n: usize) -> Result<&'static str, ()> {
    println!("called func({})", n);

    if n == 0 {
        return Ok("n == 0");
    }

    if n % 2 == 0 {
        let f = |m| -> Result<&'static str, ()> {
            if m % 3 == 0 {
                Ok("n is even and divisible by 3")
            } else {
                Ok("n is even")
            }
        };
        f(n)?;
    }

    func(n - 1)
}

fn main() {
    func(10).unwrap();
}
