use crate::even_pool::EventPool;
use crate::observer::{Event, Listener};
use crate::vec_listeners::VecListeners;
use std::any::TypeId;
use std::collections::HashMap;

pub struct ListenerMap {
    buffer_size: usize,
    listeners: HashMap<TypeId, Vec<Listener>>,
}

impl ListenerMap {
    pub fn new(buffer_size: usize) -> Self {
        Self {
            buffer_size,
            listeners: Default::default(),
        }
    }

    pub fn add(&mut self, new_listener: &Listener) {
        let rc_listener = new_listener.clone();
        let key = new_listener.event_type();

        match self.listeners.get_mut(&key) {
            Some(list) => list.push(rc_listener),
            None => {
                let new_vec_listeners = Vec::from_listener(rc_listener, self.buffer_size);
                self.listeners.insert(key, new_vec_listeners);
            }
        }
    }

    pub fn remove(&mut self, listener: &Listener) {
        self.listeners
            .entry(listener.event_type())
            .and_modify(|list| list.remove_first(listener));
    }

    pub fn call(&mut self, event: impl Event) {
        let mut event_pool = EventPool::from(event);

        while let Some(event) = event_pool.pop() {
            self.call_listeners_for(event, &mut event_pool);
        }
    }

    #[inline]
    fn call_listeners_for(&mut self, event: Box<dyn Event>, mut event_pool: &mut EventPool) {
        let event_type = (*event).type_id();

        if let Some(listeners) = self.listeners.get(&event_type) {
            listeners.notify_all(&event, &mut event_pool);
        }
    }
}
