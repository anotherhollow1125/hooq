use hooq_macros::hooq;

#[hooq]
#[hooq::method(.inspect(|_| {
    println!("tag: {}", $tag);
}))]
#[hooq::tag = "mod"]
mod tmp {
    pub fn func() -> Result<(), ()> {
        Err(())
    }
}

#[test]
fn test() {
    tmp::func().unwrap_err();
}
