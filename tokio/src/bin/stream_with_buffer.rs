// When a stream has a fast producer and slow consumer, the backlog cost more and more memory.

use futures::stream::{Stream, StreamExt};
use tokio::time::{interval, Duration};

// create a stream that emit Item with frequency for total duration_in_seconds
fn create_producer(frequency: u64, duration_in_seconds: u64) -> impl Stream<Item = u64> {
    let total = frequency * duration_in_seconds;
    let interval = interval(Duration::from_nanos(1_000_000_000 / frequency));
    futures::stream::unfold(
        (interval, total, 0u64),
        move |(mut interval, total, count)| async move {
            if count <= total {
                let _ = interval.tick().await;
                Some((count, (interval, total, count + 1)))
            } else {
                None
            }
        },
    )
}

#[tokio::main]
async fn main() {
    let clock = std::time::Instant::now();

    let mut producer = Box::pin(create_producer(1_000, 3));
    while let Some(num) = producer.next().await {
        println!("got: {}", num);
    }

    eprintln!("{}s elapsed, all done.", clock.elapsed().as_secs_f32());
}
