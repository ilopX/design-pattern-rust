use crate::even_pool::EventPool;
use crate::observer::{Event, Listener};

pub trait VecListeners<T: PartialEq> {
    fn notify_all(&self, event: &Box<dyn Event>, pool: &mut EventPool);

    fn remove_first(&mut self, element: &T);

    fn from_listener(listener: Listener, buffer_size: usize) -> Vec<Listener> {
        let mut new_list = Vec::with_capacity(buffer_size);
        new_list.push(listener);
        new_list
    }
}

impl VecListeners<Listener> for Vec<Listener> {
    fn notify_all(&self, event: &Box<dyn Event>, mut event_pool: &mut EventPool) {
        for listener in self {
            listener.notify(&event, &mut event_pool);
        }
    }

    fn remove_first(&mut self, element: &Listener) {
        let pos = self.iter().position(|a| a == element);

        if let Some(index) = pos {
            self.remove(index);
        }
    }
}
