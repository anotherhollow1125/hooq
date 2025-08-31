use hooq::hooq;

#[hooq]
#[hooq::method(.inspect(|_| {
    println!("{}", $tag);
}))]
#[hooq::tag = "root"]
fn hoge(flag: bool, flog: bool) -> Result<usize, ()> {
    #[hooq::skip]
    if flag {
        let g = || -> Result<usize, ()> {
            if flag && flog {
                Ok(1)
            } else {
                #[hooq::skip]
                Err(())
            }
        };

        g()?;

        #[hooq::skip]
        {
            let f = || -> Result<usize, ()> {
                if flag && flog {
                    #[hooq::skip_all]
                    Ok(1)
                } else {
                    #[hooq::skip]
                    Err(())
                }
            };
            let _ = f()?;

            println!("aaa");
            println!("bbb");

            #[hooq::tag = "tag A"]
            if flog {
                return Ok(0);
            }

            Ok(1)
        }
    } else {
        let g = || -> Result<usize, ()> {
            if flag && flog {
                #[hooq::skip_all]
                Ok(1)
            } else {
                Err(())
            }
        };

        g()?;

        #[hooq::skip_all]
        {
            let f = || -> Result<usize, ()> { if flag && flog { Ok(1) } else { Err(()) } };
            let _ = f()?;

            println!("aaa");
            println!("bbb");

            #[hooq::tag = "tag B"]
            if flog {
                return Ok(2);
            }

            Ok(3)
        }
    }
}

fn main() {
    hoge(true, true).unwrap();
}
