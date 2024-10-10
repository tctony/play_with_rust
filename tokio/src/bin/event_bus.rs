use futures::{stream::BoxStream, StreamExt};
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc::UnboundedSender;

pub struct EventBus<T> {
    subscribers: Mutex<Vec<UnboundedSender<Arc<T>>>>,
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

pub trait Subscribable<T: 'static + Clone + Send + Sync> {
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

    // WIP
    #[tokio::test]
    async fn test_event_bus() {
        // WIP
    }
}

fn main() {
    // do nothing
    println!("done.");
}
