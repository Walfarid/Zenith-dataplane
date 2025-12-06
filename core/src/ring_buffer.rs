use crossbeam::queue::ArrayQueue;
use std::sync::Arc;
use crate::error::{Result, ZenithError};
use crate::event::ZenithEvent;

pub struct ZenithRingBuffer {
    queue: Arc<ArrayQueue<ZenithEvent>>,
}

impl ZenithRingBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            queue: Arc::new(ArrayQueue::new(capacity)),
        }
    }

    pub fn push(&self, event: ZenithEvent) -> Result<()> {
        self.queue.push(event).map_err(|_| ZenithError::BufferFull)
    }

    pub fn pop(&self) -> Option<ZenithEvent> {
        self.queue.pop()
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}

impl Clone for ZenithRingBuffer {
    fn clone(&self) -> Self {
        Self {
            queue: self.queue.clone(),
        }
    }
}
