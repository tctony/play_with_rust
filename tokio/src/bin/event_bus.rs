use futures::{stream::BoxStream, StreamExt};
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc::UnboundedSender;

#[derive(Debug, Clone)]
pub struct EventBus<T> {
    subscribers: Arc<Mutex<Vec<UnboundedSender<Arc<T>>>>>,
}

impl<T: 'static + Send + Sync> EventBus<T> {
    pub fn new() -> Self {
        EventBus {
            subscribers: Default::default(),
        }
    }

    pub fn subscribe(&self) -> BoxStream<Arc<T>> {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();

        let mut subscribers = self.subscribers.lock().unwrap();
        subscribers.push(tx);

        tokio_stream::wrappers::UnboundedReceiverStream::new(rx).boxed()
    }

    pub fn emit(&self, val: T) {
        let event = Arc::new(val);
        let mut subscribers = self.subscribers.lock().unwrap();
        subscribers.retain(|one| {
            // only keep alived subscribers
            one.send(event.clone()).is_ok()
        });
    }
}

pub trait EventBusTrait<T: 'static + Clone + Send + Sync> {
    fn event_bus(&self) -> &EventBus<T>;

    fn subscribe_change(&self) -> BoxStream<Arc<T>> {
        self.event_bus().subscribe()
    }

    fn notify_change(&self, value: T) {
        let _ = self.event_bus().emit(value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_event_bus() {
        let bus = EventBus::<i32>::new();

        let handle = tokio::spawn({
            let bus = bus.clone();
            async move {
                for i in 0..5 {
                    println!("emit {}", i);
                    bus.emit(i);

                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                }
            }
        });

        tokio::time::sleep(tokio::time::Duration::from_millis(1_500)).await;
        tokio::spawn({
            let bus = bus.clone();
            async move {
                let mut stream = bus.subscribe();
                while let Some(event) = stream.next().await {
                    println!("recv {:?}", event);
                }
            }
        });

        let _ = handle.await;
    }

    #[derive(Debug, Clone)]
    struct EventBusWrapper {
        pub internal: EventBus<i32>,
    }

    impl EventBusTrait<i32> for EventBusWrapper {
        fn event_bus(&self) -> &EventBus<i32> {
            &self.internal
        }
    }

    #[tokio::test]
    async fn test_event_bus_trait() {
        let bus = EventBusWrapper {
            internal: EventBus::<i32>::new(),
        };

        let handle = tokio::spawn({
            let bus = bus.clone();
            async move {
                for i in 0..5 {
                    println!("emit {}", i);
                    bus.internal.emit(i);

                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                }
            }
        });

        tokio::time::sleep(tokio::time::Duration::from_millis(1_500)).await;
        tokio::spawn({
            let bus = bus.clone();
            async move {
                let mut stream = bus.subscribe_change();
                while let Some(event) = stream.next().await {
                    println!("recv {:?}", event);
                }
            }
        });

        let _ = handle.await;
    }
}

fn main() {
    // do nothing
    println!("done.");
}
