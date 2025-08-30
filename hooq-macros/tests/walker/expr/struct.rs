use hooq_macros::hooq;

#[allow(unused)]
struct Strct {
    field: u32,
}

#[allow(unused)]
enum Enm {
    Variant1 { field: u32 },
    Variant2,
}

#[allow(unused)]
union Unon {
    int32: i32,
    uint32: u32,
}

#[hooq]
fn hoge() -> Result<u32, ()> {
    Ok(10)
}

#[hooq]
#[hooq::method(.inspect(|_| {
    println!("tag: {}", $tag);
}))]
#[hooq::tag = "(no tag)"]
fn func() -> Result<(), ()> {
    let _ = Strct { field: hoge()? };

    let _ = Strct {
        #[hooq::tag = "field"]
        field: {
            if hoge()? > 100 {
                #[hooq::tag = "in field expr"]
                return Err(());
            }

            hoge()?
        },
    };

    let _ = Enm::Variant1 { field: hoge()? };

    let _ = Unon { uint32: hoge()? };

    Ok(())
}

#[test]
fn test() {
    func().unwrap();
}
