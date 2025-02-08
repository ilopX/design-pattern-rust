use crate::event::Event;
use crate::even_pool::EventPool;
use crate::observer::Listener;
use crate::vec_listeners::VecListeners;
use std::any::TypeId;
use std::collections::HashMap;

pub struct ObserverMap {
    buffer_size: usize,
    listeners_map: HashMap<TypeId, Vec<Listener>>,
}

impl ObserverMap {
    pub fn new(buffer_size: usize) -> Self {
        Self {
            buffer_size,
            listeners_map: Default::default(),
        }
    }

    pub fn add(&mut self, new_listener: &Listener) {
        let rc_listener = new_listener.clone();
        let key = new_listener.event_type();

        match self.listeners_map.get_mut(&key) {
            Some(list) => list.push(rc_listener),
            None => {
                let new_vec_listeners = Vec::from_listener(rc_listener, self.buffer_size);
                self.listeners_map.insert(key, new_vec_listeners);
            }
        }
    }

    pub fn remove(&mut self, listener: &Listener) {
        self.listeners_map
            .entry(listener.event_type())
            .and_modify(|list| list.remove_first(listener));
    }

    pub fn send(&mut self, event: impl Event) {
        let mut event_pool = EventPool::from(event);

        while let Some(event) = event_pool.pop() {
            self.send_to_all_listeners(event, &mut event_pool);
        }
    }

    #[inline]
    fn send_to_all_listeners(&mut self, event: Box<dyn Event>, mut event_pool: &mut EventPool) {
        let event_type = (*event).type_id();

        if let Some(listeners_list) = self.listeners_map.get(&event_type) {
            listeners_list.notify_all(&event, &mut event_pool);
        }
    }
}
