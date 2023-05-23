use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::btree_map::BTreeMap;
use std::mem;
use std::rc::Rc;

fn main() {
    let mut result_str = String::new();
    let mut ob = Observer::new();

    ob.subscribe(|event: &FirstEvent| {
        result_str.push_str(&format!("name: {}, ", event.name))
    });

    let print_subscriber = ob.subscribe(|event: &SecondEvent| {
        result_str.push_str(&format!("num: {}", event.num));
    });

    ob.notify(FirstEvent { name: "ilopX" });
    ob.notify(SecondEvent { num: 100 });

    ob.unsubscribe(print_subscriber);
    ob.notify(SecondEvent { num: 200 });

    assert_eq!("name: ilopX, num: 100", result_str);
}


#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn subscribe_notify() {
        let mut ob = Observer::new();
        let mut str = String::new();

        ob.subscribe(|event: &FirstEvent| str.push_str(&event.name));
        ob.notify(FirstEvent { name: "rust" });

        assert_eq!(str, "rust");
    }

    #[test]
    fn unsubscribe() {
        let mut ob = Observer::new();
        let mut it_works = false;

        let subscriber = ob.subscribe::<FirstEvent>(|_| it_works = true);
        ob.unsubscribe(subscriber);
        ob.notify(FirstEvent { name: "" });

        assert_eq!(it_works, false);
    }

    #[test]
    fn two_events() {
        let mut ob = Observer::new();
        let mut works = 0;

        ob.subscribe::<FirstEvent>(|_| works += 10);
        ob.subscribe::<SecondEvent>(|event| works += event.num);

        ob.notify(FirstEvent { name: "" });
        ob.notify(SecondEvent { num: 5 });

        assert_eq!(works, 15);
    }
}


struct Observer {
    subscribers: BTreeMap<TypeId, Vec<Subscriber>>,
}

impl Observer {
    fn new() -> Self {
        Observer {
            subscribers: BTreeMap::new()
        }
    }

    fn subscribe<E: Event>(&mut self, call: impl FnMut(&E)) -> Subscriber {
        let new_subscriber = Subscriber::new(call);
        let return_subscriber = new_subscriber.clone();

        match self.subscribers.get_mut(&new_subscriber.event_id) {
            Some(e) => {
                e.push(new_subscriber);
            }
            None => {
                self.subscribers.insert(
                    new_subscriber.event_id.clone(),
                    vec![new_subscriber],
                );
            }
        };

        return_subscriber
    }

    fn notify<E: Event>(&mut self, event: E)
    {
        let event_id = event.type_id();
        let subscribers = self.subscribers.get_mut(&event_id);

        if let Some(sub_list) = subscribers {
            for subscriber in sub_list {
                (RefCell::borrow_mut(&subscriber.fun))(&event)
            }
        }
    }

    fn unsubscribe(&mut self, subscriber: Subscriber) {
        let event_id = &subscriber.event_id;
        let subscribers = self.subscribers.get_mut(event_id);

        if let Some(list) = subscribers {
            let index = list.iter().position(|val| val == &subscriber);

            if let Some(index) = index {
                list.remove(index);
            }
        }
    }
}


type DestEvent = Rc<RefCell<dyn FnMut(&dyn Any)>>;

trait Event: Any + Sized {
    fn cast(a: &dyn Any) -> &Self {
        a.downcast_ref::<Self>().unwrap()
    }
}

trait Subject<E: Event> {
    fn update(&mut self, event: &E);
}

struct Subscriber {
    event_id: TypeId,
    fun: DestEvent,
}

impl Subscriber {
    fn new<E: Event>(call: impl FnMut(&E)) -> Self {
        Subscriber {
            event_id: TypeId::of::<E>(),
            fun: Self::convert_to_any_args(call),
        }
    }

    fn convert_to_any_args<E: Event>(call: impl FnMut(&E)) -> DestEvent {
        let call = Rc::new(RefCell::new(call));
        unsafe {
            mem::transmute::<Rc<RefCell<dyn FnMut(&E)>>, DestEvent>(call)
        }
    }
}

impl Clone for Subscriber {
    fn clone(&self) -> Self {
        Subscriber {
            event_id: self.event_id.clone(),
            fun: Rc::clone(&self.fun),
        }
    }
}

impl PartialEq for Subscriber {
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

