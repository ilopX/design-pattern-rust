use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::btree_map::BTreeMap;
use std::mem;
use std::rc::Rc;

fn main() {
    let result_str = RefCell::new(String::new());
    let mut ob = Observer::new();

    {
        ob.subscribe(|event: &FirstEvent| {
            result_str.borrow_mut().push_str(&format!("name: {} \n", event.name))
        });

        ob.subscribe(|event: &SecondEvent| {
            result_str.borrow_mut().push_str(&format!("num: {} \n", event.num));
        });
    }

    let print_subscriber = ob.subscribe::<PrintResultEvent>(|_| {
        println!("{}", result_str.borrow());
    });

    ob.notify(FirstEvent { name: "ilopX" });
    ob.notify(SecondEvent { num: 100 });
    ob.notify(SecondEvent { num: 200 });
    ob.notify(PrintResultEvent {});

    ob.unsubscribe(print_subscriber);
    ob.notify(PrintResultEvent {});
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

        let subscriber = ob.subscribe::<FirstEvent>(|_| it_works.set( true));
        ob.unsubscribe(subscriber);
        ob.notify(FirstEvent { name: "" });

        assert_eq!(it_works.get(), false);
    }

    #[test]
    fn two_events() {
        let works = Cell::new(0);
        let mut ob = Observer::new();

        ob.subscribe::<FirstEvent>(|_| works.set(works.get() + 10) );
        ob.subscribe::<SecondEvent>(|event| works.set(works.get() + event.num));

        ob.notify(FirstEvent { name: "" });
        ob.notify(SecondEvent { num: 5 });

        assert_eq!(works.get(), 15);
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

    fn unsubscribe(&mut self, subscriber: Subscriber<'a>) {
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


type DestEvent<'a> = Rc<RefCell<dyn FnMut(&dyn Any) + 'a>>;

trait Event: Any + Sized {
    fn cast(a: &dyn Any) -> &Self {
        a.downcast_ref::<Self>().unwrap()
    }
}

struct Subscriber<'a> {
    event_id: TypeId,
    fun: DestEvent<'a>,
}

impl<'a> Subscriber<'a> {
    fn new<E: Event>(call: impl FnMut(&E) + 'a) -> Self {
        Self {
            event_id: TypeId::of::<E>(),
            fun: Self::convert_to_any_args(call),
        }
    }

    fn convert_to_any_args<E: Event>(call: impl FnMut(&E)) -> DestEvent<'a> {
        let call = Rc::new(RefCell::new(call));
        unsafe {
            mem::transmute::<Rc<RefCell<dyn FnMut(&E)>>, DestEvent>(call)
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

struct PrintResultEvent {}

impl Event for PrintResultEvent {}
