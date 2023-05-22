use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

fn main() {
    let mut accumulator = vec![];
    let mut ob = Observer::<i32>::new();

    let subscriber = ob.subscribe(|val| {
        accumulator.push(val.clone());
    });

    ob.notify(1);
    ob.notify(2);
    ob.unsubscribe(subscriber);
    ob.notify(3);
    ob.notify(4);

    assert_eq!(accumulator.as_slice(), &[1, 2]);
}


struct Observer<T> {
    subs: Vec<Subscriber<T>>,
}

impl<T> Observer<T> {
    fn new() -> Self {
        Self {
            subs: vec![],
        }
    }

    fn subscribe(&mut self, call: impl FnMut(&T)) -> Subscriber<T> {
        let subscriber = Subscriber::new(call);
        let return_subscriber = subscriber.clone();
        self.subs.push(subscriber);

        return_subscriber
    }

    fn unsubscribe(&mut self, subscriber: Subscriber<T>) {
        self.subs.retain(|val| val != &subscriber);
    }

    fn notify(&mut self, new_value: T) {
        for subscriber in self.subs.iter() {
            subscriber.call(&new_value);
        }
    }
}


type SubscriberCall<T> = Rc<RefCell<dyn FnMut(&T)>>;

struct Subscriber<T> {
    call: SubscriberCall<T>,
}

impl<T> Subscriber<T> {
    fn new(call: impl FnMut(&T)) -> Self {
        Self {
            call: Self::reset_lifetime(call),
        }
    }

    fn call(&self, val: &T) {
        self.call.borrow_mut()(val);
    }

    fn reset_lifetime<E>(call: impl FnMut(&E)) -> SubscriberCall<E> {
        let call = Rc::new(RefCell::new(call));
        unsafe {
            mem::transmute::<Rc<RefCell<dyn FnMut(&E)>>, SubscriberCall<E>>(call)
        }
    }
}

impl<T> Clone for Subscriber<T> {
    fn clone(&self) -> Self {
        Subscriber {
            call: Rc::clone(&self.call)
        }
    }
}

impl<T> PartialEq<Self> for Subscriber<T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.call, &other.call)
    }
}
