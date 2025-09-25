use hooq_macros::hooq;

#[hooq]
unsafe fn hoge() -> Result<(), ()> {
    Err(())
}

#[hooq]
#[hooq::method(.inspect(|_| {
    println!("tag: {}", $tag);
}))]
#[hooq::tag = "(no tag)"]
fn func(flags: Vec<bool>) -> Result<(), ()> {
    let mut flags = flags.into_iter();

    unsafe {
        #[hooq::tag = "in unsafe"]
        hoge()?;

        if flags.next().unwrap_or(false) {
            return Err(());
        }

        let _ = || {
            #[hooq::tag = "in closure"]
            hoge()?;

            hoge()
        };

        if flags.next().unwrap_or(false) {
            return hoge();
        }
    }

    Err(())
}

#[test]
fn test() {
    func(vec![false, false]).unwrap_err();
}
