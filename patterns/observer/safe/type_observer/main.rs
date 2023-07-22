use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::btree_map::BTreeMap;
use std::mem;
use std::rc::Rc;

fn main() {
    let result_str = RefCell::new(String::new());
    let mut ob = Observer::new();

    ob.subscribe(|event: &FirstEvent| {
        result_str.borrow_mut().push_str(&format!("name: {}, ", event.name))
    });

    let second_subscriber = ob.subscribe(|event: &SecondEvent| {
        result_str.borrow_mut().push_str(&format!("num: {}", event.num));
    });

    ob.notify(FirstEvent { name: "ilopX" });
    ob.notify(SecondEvent { num: 100 });

    ob.unsubscribe(second_subscriber);
    ob.notify(SecondEvent { num: 200 }); // not be called

    assert_eq!(*result_str.borrow(), "name: ilopX, num: 100");
}


#[cfg(test)]
mod tests {
    use std::cell::Cell;

    use crate::*;

    #[test]
    fn subscribe_notify() {
        let str = RefCell::new(String::new());
        let mut ob = Observer::new();

        ob.subscribe(|event: &FirstEvent| str.borrow_mut().push_str(&event.name));
        ob.notify(FirstEvent { name: "rust" });

        assert_eq!(*str.borrow(), "rust");
    }

    #[test]
    fn unsubscribe() {
        let it_works = Cell::new(false);
        let mut ob = Observer::new();

        let subscriber = ob.subscribe::<FirstEvent>(|_| it_works.set(true));
        ob.unsubscribe(subscriber);
        ob.notify(FirstEvent { name: "" });

        assert_eq!(it_works.get(), false);
    }

    #[test]
    fn two_events() {
        let counter = Cell::new(0);
        let mut ob = Observer::new();

        ob.subscribe::<FirstEvent>(|_| counter.set(counter.get() + 10));
        ob.subscribe::<SecondEvent>(|event| counter.set(counter.get() + event.num));

        ob.notify(FirstEvent { name: "" });
        ob.notify(SecondEvent { num: 5 });

        assert_eq!(counter.get(), 15);
    }
}


struct Observer<'a> {
    subscribers: BTreeMap<TypeId, Vec<Subscriber<'a>>>,
}

impl<'a> Observer<'a> {
    fn new() -> Self {
        Observer {
            subscribers: BTreeMap::new()
        }
    }

    fn subscribe<E: Event>(&mut self, call: impl FnMut(&E) + 'a) -> Subscriber<'a> {
        let new_subscriber = Subscriber::new(call);
        let return_subscriber = new_subscriber.clone();
        self.add(new_subscriber);

        return_subscriber
    }

    fn notify<E: Event>(&mut self, event: E) {
        if let Some(subscribers) = self.get_subscribers(&event.type_id()) {
            for subscriber in subscribers {
                subscriber.call(&event);
            }
        }
    }

    fn unsubscribe(&mut self, subscriber: Subscriber<'a>) {
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
            Some(existing_list) => {
                existing_list.push(new_subscriber)
            }
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


type SubscriberCall<'a> = Rc<RefCell<dyn FnMut(&dyn Any) + 'a>>;

trait Event: Any + Sized {
    fn cast(a: &dyn Any) -> &Self {
        a.downcast_ref::<Self>().unwrap()
    }
}

struct Subscriber<'a> {
    event_id: TypeId,
    fun: SubscriberCall<'a>,
}

impl<'a> Subscriber<'a> {
    fn new<E: Event>(call: impl FnMut(&E) + 'a) -> Self {
        Self {
            event_id: TypeId::of::<E>(),
            fun: Self::convert_to_any_args(call),
        }
    }

    #[inline]
    fn call<E: Event>(&self, event: &E) {
        (RefCell::borrow_mut(&self.fun))(event);
    }

    fn convert_to_any_args<E: Event>(call: impl FnMut(&E)) -> SubscriberCall<'a> {
        let call = Rc::new(RefCell::new(call));
        unsafe {
            mem::transmute::<Rc<RefCell<dyn FnMut(&E)>>, SubscriberCall<'a>>(call)
        }
    }
}

impl<'a> Clone for Subscriber<'a> {
    fn clone(&self) -> Self {
        Subscriber {
            event_id: self.event_id.clone(),
            fun: Rc::clone(&self.fun),
        }
    }
}

impl<'a> PartialEq for Subscriber<'a> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.fun, &other.fun)
    }
}

struct FirstEvent {
    name: &'static str,
}

impl Event for FirstEvent {}


struct SecondEvent {
    num: i32,
}

impl Event for SecondEvent {}
