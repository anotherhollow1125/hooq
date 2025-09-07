use hooq_macros::hooq;

#[hooq]
#[hooq::method(.inspect(|_| {
    println!("tag: {}", $tag);
}))]
#[hooq::tag = "mod"]
mod tmp {
    pub fn func() -> Result<(), ()> {
        Ok(())
    }
}

#[test]
fn test() {
    tmp::func().unwrap();
}
