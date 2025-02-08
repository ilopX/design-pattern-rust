use std::collections::VecDeque;
use crate::event::Event;

pub struct EventPool {
    events: VecDeque<Box<dyn Event>>,
}

impl EventPool {
    pub fn new() -> Self {
        Self {
            events: VecDeque::with_capacity(10),
        }
    }

    pub fn from(event: impl Event) -> Self {
        let mut events = VecDeque::<Box<dyn Event>>::new();
        events.push_back(Box::new(event));

        Self {
            events,
            ..EventPool::new()
        }
    }

    pub fn send(&mut self, event: impl Event) {
        let event = Box::new(event);
        self.events.push_back(event);
    }

    pub fn pop(&mut self) -> Option<Box<dyn Event>> {
        self.events.pop_front()
    }
}
