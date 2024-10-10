use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};

async fn notifier(tx: mpsc::Sender<String>) {
    // 模拟发送通知
    for i in 1..=5 {
        let msg = format!("Notification {}", i);
        // 发送通知，不关心是否送达
        let _ = tx.send(msg).await;
        sleep(Duration::from_secs(1)).await;
    }
}

async fn receiver(mut rx: mpsc::Receiver<String>) {
    // 模拟接收通知
    while let Some(msg) = rx.recv().await {
        println!("Received: {}", msg);
    }
}

#[tokio::main]
async fn main() {
    // 创建一个异步通道
    let (tx, rx) = mpsc::channel(10);

    // 启动通知发送任务
    tokio::spawn(notifier(tx));

    // we should not miss the first notify, since the 'channel' will cache all pending messages.
    sleep(Duration::from_millis(1_500)).await;

    // 启动通知接收任务
    receiver(rx).await;
}
