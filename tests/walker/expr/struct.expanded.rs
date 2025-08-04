use hooq::hooq;
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
fn hoge() -> Result<u32, ()> {
    Ok(10)
        .inspect_err(|e| {
            {
                ::std::io::_eprint(
                    format_args!(
                        "{0:?} @ path: {1}, line: {2}\n", e,
                        "<hooq_root>/tests/walker/expr/struct.rs", 22usize
                    ),
                );
            };
        })
}
fn func() -> Result<(), ()> {
    let _ = Strct {
        field: hoge()
            .inspect(|_| {
                {
                    ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
                };
            })?,
    };
    let _ = Strct {
        field: {
            if hoge()
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0}\n", "field"));
                    };
                })? > 100
            {
                return Err(())
                    .inspect(|_| {
                        {
                            ::std::io::_print(
                                format_args!("tag: {0}\n", "in field expr"),
                            );
                        };
                    });
            }
            hoge()
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0}\n", "field"));
                    };
                })?
        },
    };
    let _ = Enm::Variant1 {
        field: hoge()
            .inspect(|_| {
                {
                    ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
                };
            })?,
    };
    let _ = Unon {
        uint32: hoge()
            .inspect(|_| {
                {
                    ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
                };
            })?,
    };
    Ok(())
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0}\n", "(no tag)"));
            };
        })
}
