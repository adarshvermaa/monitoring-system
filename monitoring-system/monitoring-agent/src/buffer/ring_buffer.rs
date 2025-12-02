use crossbeam::queue::ArrayQueue;
use monitoring_common::{Event, MonitoringError};
use std::sync::Arc;

/// Lock-free ring buffer for event storage
pub struct RingBuffer {
    queue: Arc<ArrayQueue<Event>>,
}

impl RingBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            queue: Arc::new(ArrayQueue::new(capacity)),
        }
    }

    /// Push an event into the buffer
    /// Returns error if buffer is full
    pub fn push(&self, event: Event) -> Result<(), MonitoringError> {
        self.queue
            .push(event)
            .map_err(|_| MonitoringError::BufferOverflow)
    }

    /// Pop an event from the buffer
    pub fn pop(&self) -> Option<Event> {
        self.queue.pop()
    }

    /// Drain up to N events from the buffer
    pub fn drain(&self, max_count: usize) -> Vec<Event> {
        let mut events = Vec::with_capacity(max_count.min(self.len()));
        
        for _ in 0..max_count {
            match self.pop() {
                Some(event) => events.push(event),
                None => break,
            }
        }

        events
    }

    /// Get current buffer length
    pub fn len(&self) -> usize {
        self.queue.len()
    }

    /// Check if buffer is empty
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    /// Check if buffer is full
    pub fn is_full(&self) -> bool {
        self.queue.is_full()
    }

    /// Get buffer capacity
    pub fn capacity(&self) -> usize {
        self.queue.capacity()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use monitoring_common::{LogEvent, LogLevel};
    use std::collections::HashMap;

    #[test]
    fn test_ring_buffer() {
        let buffer = RingBuffer::new(10);
        
        assert_eq!(buffer.len(), 0);
        assert!(buffer.is_empty());
        
        let event = Event::Log(LogEvent {
            timestamp: 123,
            source: "test".to_string(),
            level: LogLevel::Info,
            message: "test".to_string(),
            fields: HashMap::new(),
            tags: vec![],
        });

        buffer.push(event.clone()).unwrap();
        assert_eq!(buffer.len(), 1);
        
        let popped = buffer.pop().unwrap();
        assert_eq!(buffer.len(), 0);
    }

    #[test]
    fn test_buffer_overflow() {
        let buffer = RingBuffer::new(2);
        
        let event = Event::Log(LogEvent {
            timestamp: 123,
            source: "test".to_string(),
            level: LogLevel::Info,
            message: "test".to_string(),
            fields: HashMap::new(),
            tags: vec![],
        });

        buffer.push(event.clone()).unwrap();
        buffer.push(event.clone()).unwrap();
        
        // Third push should fail
        assert!(buffer.push(event).is_err());
    }

    #[test]
    fn test_drain() {
        let buffer = RingBuffer::new(10);
        
        for i in 0..5 {
            let event = Event::Log(LogEvent {
                timestamp: i,
                source: "test".to_string(),
                level: LogLevel::Info,
                message: format!("message {}", i),
                fields: HashMap::new(),
                tags: vec![],
            });
            buffer.push(event).unwrap();
        }

        let events = buffer.drain(3);
        assert_eq!(events.len(), 3);
        assert_eq!(buffer.len(), 2);
    }
}
