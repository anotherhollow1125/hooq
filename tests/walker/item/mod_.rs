use hooq::hooq;

#[hooq]
#[hooq::method(.map(|v| {
    println!("tag: {:?}", $tag);
    v
}))]
fn func() -> Result<(), ()> {
    #[hooq::tag("mod")]
    mod tmp {
        fn func() -> Result<(), ()> {
            Ok(())
        }
    }

    Ok(())
}

fn main() {
    func().unwrap();
}
