use hooq_macros::hooq;

#[hooq]
fn get_bool(i: &mut usize) -> Result<bool, ()> {
    *i += 1;

    Ok(*i < 5)
}

#[hooq]
#[hooq::method(.inspect(|_| {
    println!("tag: {}", $tag);
}))]
fn func() -> Result<(), ()> {
    let mut i = 0;

    #[hooq::tag("while")]
    while get_bool(&mut i)? {
        let mut j = 0;

        let _ = get_bool(&mut j)?;

        #[hooq::tag("1")]
        if !get_bool(&mut j)? {
            get_bool(&mut j)?;

            return Ok(());
        }

        #[hooq::tag("2")]
        if !get_bool(&mut j)? {
            get_bool(&mut j)?;

            return get_bool(&mut j).map(|_| ());
        }

        #[hooq::tag("3")]
        if !get_bool(&mut j)? {
            get_bool(&mut j)?;

            break;
        }
    }

    Ok(())
}

#[test]
fn test() {
    func().unwrap();
}
