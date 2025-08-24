use hooq_macros::hooq;
use tokio::time::{Duration, sleep};
async fn func() -> Result<(), ()> {
    let res = tokio::spawn(async {
            sleep(Duration::from_millis(10)).await;
            Result::<(), ()>::Ok(())
                .inspect(|_| {
                    {
                        ::std::io::_print(format_args!("tag: {0}\n", "async function"));
                    };
                })
        })
        .await;
    res.map_err(|_| ())
        .inspect(|_| {
            {
                ::std::io::_print(format_args!("tag: {0}\n", "async function"));
            };
        })?
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
