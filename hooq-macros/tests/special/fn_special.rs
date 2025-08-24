use hooq_macros::hooq;

// 関数周りについて重点的なテスト
// return.rs がベース

#[hooq]
#[hooq::method(.inspect(|_| {
    println!("tag: {}", $tag);
}))]
fn func(flag: bool, flog: bool) -> Result<(), ()> {
    #[hooq::tag("return")]
    if flag {
        return Ok(());
    }

    #[hooq::tag("not result")]
    fn hoge(flag: bool) -> u32 {
        if flag {
            return 1;
        }
        0
    }

    let _ = hoge(flag);

    #[hooq::tag("nest")]
    fn fuga(flag: bool) -> u32 {
        let a = |b| if b { Ok(10) } else { Err(()) };
        let b = || -> Result<u32, ()> { Ok(10) };
        let c = || -> Result<u32, ()> { b() };
        let d = || -> Result<u32, ()> {
            if flag {
                #[hooq::tag("return in deep")]
                {
                    return c();
                }
            }
            fn dd() -> u32 {
                10
            }
            let ddd = || {
                if flag {
                    return hoge(true);
                }

                let dddd = || {
                    if flag {
                        return b();
                    }

                    let ddddd = || {
                        let res = c()? + 10;

                        Result::<u32, ()>::Ok(res)
                    };
                    ddddd()
                };

                20 + dddd().unwrap()
            };

            Ok(dd() + ddd())
        };
        let e = || {
            if flag {
                Ok(a(true)? + b()? + c()? + d()?)
            } else {
                Err(())
            }
        };

        let v = e().unwrap();

        v + 10
    }

    let _ = fuga(flog);

    Ok(())
}

#[test]
fn test() {
    func(true, true).unwrap();
}
