use hooq::hooq;

#[hooq]
#[hooq::method(.inspect_err(|e| {
    if *e > $threshold {
        eprintln!("dif: {} with tag: {}", ($closure)(e, $threshold), $tag);
    }
}))]
#[hooq::tag = "threshold_check"]
#[hooq::threshold = 5]
#[hooq::closure = |e, t| e - t]
fn func(val: usize) -> Result<(), usize> {
    Err(val)
}

fn main() {
    let _ = func(10).unwrap_err();
}
