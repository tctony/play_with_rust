use tokio::sync::watch;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = watch::channel(false);

    tokio::spawn(async move {
        if rx.wait_for(|v| *v).await.is_ok() {
            println!("did init");
        }
    });

    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    println!("send false");
    tx.send(false).unwrap();
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    println!("send false");
    tx.send(false).unwrap();
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    println!("send true");
    tx.send(true).unwrap();
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    println!("end.");
}
