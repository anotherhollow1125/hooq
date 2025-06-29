use hooq::hooq;
use tokio::time::Duration;
use tokio::time::sleep;
async fn hoge() -> Result<usize, ()> {
    sleep(Duration::from_millis(10)).await;
    Ok(10)
}
async fn func(n: usize) -> Result<(), ()> {
    let tasks = (0..n)
        .map(|i| {
            tokio::spawn(async move {
                let _n = hoge()
                    .await
                    .inspect(|_| {
                        {
                            ::std::io::_print(format_args!("tag: {0}\n", "async block"));
                        };
                    })?;
                sleep(Duration::from_millis(100 * (i + 1) as u64)).await;
                {
                    ::std::io::_print(format_args!("Task {0} completed\n", i));
                };
                if i % 2 == 0 {
                    return Err(())
                        .inspect(|_| {
                            {
                                ::std::io::_print(
                                    format_args!("tag: {0}\n", "async block"),
                                );
                            };
                        });
                }
                Ok(())
                    .inspect(|_| {
                        {
                            ::std::io::_print(format_args!("tag: {0}\n", "async block"));
                        };
                    })
            })
        })
        .collect::<Vec<_>>();
    futures::future::try_join_all(tasks)
        .await
        .map_err(|e| {
            {
                ::std::io::_eprint(format_args!("An error occurred: {0:?}\n", e));
            };
        })
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0}\n", "async function"));
            };
        })?;
    Ok(())
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0}\n", "async function"));
            };
        })
}
