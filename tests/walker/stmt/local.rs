use hooq::hooq;

#[hooq]
#[hooq::method(.map(|v| {
    println!("tag: {:?}", $tag);
    v
}))]
fn func() -> Result<(), ()> {
    let _ = #[hooq::tag("outer")]
    (|| {
        #[hooq::tag("inner")]
        Ok(())
    })()?;

    let n = 1;

    #[hooq::tag("else")]
    let 1 = n else {
        return Err(());
    };

    Ok(())
}

fn main() {
    func().unwrap();
}
