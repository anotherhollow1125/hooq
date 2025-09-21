use hooq_macros::hooq;
use tokio::time::{Duration, sleep};

#[hooq]
#[hooq::tag = "async function"]
#[hooq::method(.inspect(|_| {
    println!("tag: {}", $tag);
}))]
#[hooq::tail_expr_idents("Ok", "Err")]
async fn func() -> Result<(), ()> {
    let res = tokio::spawn(async {
        sleep(Duration::from_millis(10)).await;

        Result::<(), ()>::Err(())
    })
    .await;

    res.map_err(|_| ())??;

    Err(())
}

#[tokio::test]
async fn test() {
    let _ = func().await;
}
