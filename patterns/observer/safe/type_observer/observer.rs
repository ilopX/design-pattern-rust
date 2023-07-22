use crate::subscriber::Subscriber;
use crate::Event;
use std::any::TypeId;
use std::collections::BTreeMap;

pub struct Observer<'a> {
    subscribers: BTreeMap<TypeId, Vec<Subscriber<'a>>>,
}

impl<'a> Observer<'a> {
    pub fn new() -> Self {
        Observer {
            subscribers: BTreeMap::new(),
        }
    }

    pub fn subscribe<E: Event>(&mut self, call: impl FnMut(&E) + 'a) -> Subscriber<'a> {
        let new_subscriber = Subscriber::new(call);
        let return_subscriber = new_subscriber.clone();
        self.add(new_subscriber);

        return_subscriber
    }

    pub fn notify<E: Event>(&mut self, event: E) {
        if let Some(subscribers) = self.get_subscribers(&event.type_id()) {
            for subscriber in subscribers {
                subscriber.call(&event);
            }
        }
    }

    pub fn unsubscribe(&mut self, subscriber: Subscriber<'a>) {
        if let Some(subscribers) = self.get_subscribers(&subscriber.event_id) {
            let index = subscribers.iter().position(|val| val == &subscriber);

            if let Some(index) = index {
                subscribers.remove(index);
            }
        }
    }

    #[inline]
    fn add(&mut self, new_subscriber: Subscriber<'a>) {
        let event_id = new_subscriber.event_id;

        match self.get_subscribers(&event_id) {
            Some(existing_list) => existing_list.push(new_subscriber),
            None => {
                let new_list = vec![new_subscriber];
                self.subscribers.insert(event_id.clone(), new_list);
            }
        };
    }

    #[inline]
    fn get_subscribers(&mut self, event_id: &TypeId) -> Option<&mut Vec<Subscriber<'a>>> {
        self.subscribers.get_mut(event_id)
    }
}
