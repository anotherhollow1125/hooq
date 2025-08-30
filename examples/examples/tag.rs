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
#[hooq::tag = "top level tag"]
fn hoge(flag: bool, tag: Tag) -> Result<(), ()> {
    if flag {
        return Ok(());
    }

    #[hooq::tag = "match"]
    match tag {
        Tag::A =>
        {
            #[hooq::tag = "a"]
            Ok(())
        }
        Tag::B =>
        {
            #[hooq::tag = "b"]
            Ok(())
        }
        Tag::C =>
        {
            #[hooq::tag = "c"]
            Ok(())
        }
        Tag::NoOne => Ok(()),
        Tag::Err =>
        {
            #[hooq::tag = "err"]
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
#[hooq::tag = "(no tag)"]
fn main() -> Result<(), ()> {
    hoge(true, Tag::A)?;

    // (no tag)
    hoge(false, Tag::A)?;

    eprintln!("\n=========\n");

    #[hooq::tag = "outer 1"]
    hoge(false, Tag::B)?;

    eprintln!("\n=========\n");

    // (no tag)
    hoge(false, Tag::C)?;

    eprintln!("\n=========\n");

    hoge(false, Tag::NoOne)?;

    eprintln!("\n=========\n");

    #[hooq::tag = "outer 2"]
    hoge(false, Tag::NoOne)?;

    eprintln!("\n=========\n");

    // (no tag)
    // リセットされているかの確認
    hoge(false, Tag::NoOne)?;

    eprintln!("\n=========\n");

    #[hooq::tag = "outer 3"]
    {
        hoge(false, Tag::A)?;

        eprintln!("\n=========\n");

        // (no tag) -> outer 3
        fuga()?;

        eprintln!("\n=========\n");

        {
            hoge(false, Tag::B)?;
        }

        eprintln!("\n=========\n");

        #[hooq::tag = "outer 4"]
        hoge(false, Tag::C)?;
    }

    eprintln!("\n=========\n");

    let Err(()) = hoge(false, Tag::Err) else {
        unreachable!();
    };

    eprintln!("\n=========\n");

    #[hooq::tag = "tail expr"]
    Ok(())
}
