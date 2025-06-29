use hooq::hooq;

enum Tag {
    A,
    B,
    C,
    NoOne,
    Err,
}

#[hooq]
#[hooq::method(.map(|v| {
    eprintln!("tag: {} @ {} {}", $tag, $line, $expr);
    v
}))]
fn hoge(tag: Tag) -> Result<(), ()> {
    #[hooq::tag("match")]
    match tag {
        Tag::A =>
        {
            #[hooq::tag("a")]
            Ok(())
        }
        Tag::B =>
        {
            #[hooq::tag("b")]
            Ok(())
        }
        Tag::C =>
        {
            #[hooq::tag("c")]
            Ok(())
        }
        Tag::NoOne => Ok(()),
        Tag::Err =>
        {
            #[hooq::tag("err")]
            #[hooq::method(.map_err(|e| {
                eprintln!("tag: {} @ {}", $tag, $line);
                e
            }))]
            Err(())
        }
    }
}

fn fuga() -> Result<(), ()> {
    Ok(())
}

#[hooq]
#[hooq::method(.map(|v| {
    eprintln!("tag: {} @ {} {}", $tag, $line, $expr);
    v
}))]
fn main() -> Result<(), ()> {
    // (no tag)
    hoge(Tag::A)?;

    eprintln!("\n=========\n");

    #[hooq::tag("outer 1")]
    hoge(Tag::B)?;

    eprintln!("\n=========\n");

    // (no tag)
    hoge(Tag::C)?;

    eprintln!("\n=========\n");

    hoge(Tag::NoOne)?;

    eprintln!("\n=========\n");

    #[hooq::tag("outer 2")]
    hoge(Tag::NoOne)?;

    eprintln!("\n=========\n");

    // (no tag)
    // リセットされているかの確認
    hoge(Tag::NoOne)?;

    eprintln!("\n=========\n");

    #[hooq::tag("outer 3")]
    {
        hoge(Tag::A)?;

        eprintln!("\n=========\n");

        // (no tag) -> outer 3
        fuga()?;

        eprintln!("\n=========\n");

        {
            hoge(Tag::B)?;
        }

        eprintln!("\n=========\n");

        #[hooq::tag("outer 4")]
        hoge(Tag::C)?;
    }

    eprintln!("\n=========\n");

    let Err(()) = hoge(Tag::Err) else {
        unreachable!();
    };

    eprintln!("\n=========\n");

    #[hooq::tag("tail expr")]
    Ok(())
}
