use hooq::hooq;

#[hooq]
#[hooq::method(.inspect_err(|e| {
    #[allow(clippy::nonminimal_bool)]
    if *e > $threshold && !$skip {
        eprintln!("dif: {} with tag: {}", ($closure)(e, $threshold), $tag);
    }
}))]
#[hooq::var(tag = "threshold_check")]
#[hooq::binding(skip = false)]
#[hooq::threshold = 5]
#[hooq::closure = |e, t| e - t]
fn func(val: usize) -> Result<(), usize> {
    #[hooq::threshold = 9]
    if val > 9 {
        #[hooq::method(.inspect_err(|e| {
            #[allow(clippy::nonminimal_bool)]
            if *e > $threshold && !$skip {
                eprintln!(
                    "dif: {} with tag: {} with hoge: {}",
                    ($closure)(e, $threshold),
                    $tag,
                    $hoge
                );
            }
        }))]
        #[hooq::hoge = "hoge"]
        if val > 20 {
            return Err(val);
        }

        return Err(val);
    }

    Err(val)
}

fn main() {
    let _ = func(10).unwrap_err();
}
