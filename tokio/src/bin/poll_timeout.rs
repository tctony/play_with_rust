use futures::{stream::Stream, Future};
use std::{pin::Pin, task::Poll};
use tokio::time::{sleep, Duration, Sleep};
use tokio_stream::StreamExt;

type T = i64;

struct SomeStream<S: Stream<Item = T>> {
    source: Pin<Box<S>>,
    timeout: Option<Pin<Box<Sleep>>>,
}

impl<S: Stream<Item = T>> SomeStream<S> {
    fn new(source: S) -> Self {
        SomeStream {
            source: Box::pin(source),
            timeout: None,
        }
    }
}

impl<S: Stream<Item = T>> Stream for SomeStream<S> {
    type Item = T;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context,
    ) -> std::task::Poll<Option<Self::Item>> {
        let this = self.get_mut();

        println!("poll_next");

        loop {
            match this.source.as_mut().poll_next(cx) {
                Poll::Ready(Some(num)) => {
                    let _ = this.timeout.take();
                    return Poll::Ready(Some(num));
                }
                Poll::Pending => {
                    if this.timeout.is_none() {
                        let sleep = sleep(Duration::from_secs(5));
                        this.timeout = Some(Box::pin(sleep));
                    }

                    // 等待超时
                    match this.timeout.as_mut().unwrap().as_mut().poll(cx) {
                        Poll::Pending => {
                            return Poll::Pending;
                        }
                        _ => {
                            println!("timeout");
                            let _ = this.timeout.take();
                            return Poll::Ready(Some(-1));
                        }
                    }
                }
                Poll::Ready(None) => {
                    let _ = this.timeout.take();
                    return Poll::Ready(None);
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // get inverval from args
    let interval = std::env::args()
        .nth(1)
        .unwrap_or("10".to_string())
        .parse::<u64>()
        .unwrap();

    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
    tokio::spawn(async move {
        let mut count: T = 1;
        loop {
            // emit a value every 10 second
            tokio::time::sleep(tokio::time::Duration::from_secs(interval)).await;
            println!("tick {}", count);
            tx.send(count).unwrap();
            count += 1;
        }
    });

    let stream = tokio_stream::wrappers::UnboundedReceiverStream::new(rx);

    let mut stream = SomeStream::new(stream);

    while let Some(num) = stream.next().await {
        println!("stream next: {}", num);
    }
}
