use std::{pin::Pin, task::Poll};

use futures::stream::Stream;
use tokio_stream::StreamExt;

struct SomeStream<S: Stream<Item = u64>> {
    source: Pin<Box<S>>,
}

impl<S: Stream<Item = u64>> SomeStream<S> {
    fn new(source: S) -> Self {
        SomeStream {
            source: Box::pin(source),
        }
    }
}

impl<S: Stream<Item = u64>> Stream for SomeStream<S> {
    type Item = u64;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context,
    ) -> std::task::Poll<Option<Self::Item>> {
        let this = self.get_mut();

        println!("poll_next");

        loop {
            match this.source.as_mut().poll_next(cx) {
                Poll::Pending => {
                    break;
                }
                _ => {}
            }
        }

        Poll::Pending
    }
}

#[tokio::main]
async fn main() {
    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
    tokio::spawn(async move {
        let mut count: u64 = 1;
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            println!("tick {}", count);
            count += 1;
            tx.send(count).unwrap();
        }
    });

    let stream = tokio_stream::wrappers::UnboundedReceiverStream::new(rx);
    let mut stream = SomeStream::new(stream);
    while let Some(num) = stream.next().await {
        println!("got: {}", num);
    }
}
