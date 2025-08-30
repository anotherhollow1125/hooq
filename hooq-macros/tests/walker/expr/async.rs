use hooq_macros::hooq;
use tokio::time::{Duration, sleep};

async fn hoge() -> Result<usize, ()> {
    sleep(Duration::from_millis(10)).await;

    Ok(10)
}

#[hooq]
#[hooq::tag = "async function"]
#[hooq::method(.inspect(|_| {
    println!("tag: {}", $tag);
}))]
async fn func(n: usize) -> Result<(), ()> {
    let tasks = (0..n)
        .map(|i| {
            #[hooq::tag = "async block"]
            tokio::spawn(async move {
                let _n = hoge().await?;

                sleep(Duration::from_millis(100 * (i + 1) as u64)).await;
                println!("Task {i} completed");

                if i % 2 == 0 {
                    return Err(());
                }

                Ok(())
            })
        })
        .collect::<Vec<_>>();

    futures::future::try_join_all(tasks).await.map_err(|e| {
        eprintln!("An error occurred: {e:?}");
    })?;

    Ok(())
}

#[tokio::test]
async fn test() {
    let _ = func(5).await;
}
