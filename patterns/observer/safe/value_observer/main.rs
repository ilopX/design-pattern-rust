use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let accumulator = RefCell::new(vec![]);
    let mut ob = ValueObserver::new(0);

    let subscriber = ob.subscribe(|val| {
        accumulator.borrow_mut().push(*val);
    });

    ob.set(1);
    ob.set(2);
    ob.unsubscribe(subscriber);
    ob.set(3);

    assert_eq!(accumulator.borrow().as_slice(), &[1, 2]);
    assert_eq!(ob.get(), &2);
}


struct ValueObserver<'a, T> {
    val: T,
    subscribers: Vec<Subscriber<'a, T>>,
}

impl<'a, T> ValueObserver<'a, T> {
    fn new(val: T) -> Self {
        Self {
            val,
            subscribers: vec![],
        }
    }

    fn subscribe(&mut self, call: impl FnMut(&T) + 'a) -> Subscriber<'a, T> {
        let subscriber = Subscriber::new(call);
        let return_subscriber = subscriber.clone();
        self.subscribers.push(subscriber);

        return_subscriber
    }

    fn unsubscribe(&mut self, subscriber: Subscriber<'a, T>) {
        self.subscribers.retain(|val| val != &subscriber);
    }

    fn set(&mut self, new_value: T) {
        for subscriber in self.subscribers.iter() {
            subscriber.call(&new_value);
        }

        self.val = new_value;
    }

    fn get(&self) -> &T {
        &self.val
    }
}


type SubscriberCall<'a, T> = Rc<RefCell<dyn FnMut(&T) + 'a>>;

struct Subscriber<'a, T> {
    call: SubscriberCall<'a, T>,
}

impl<'a, T> Subscriber<'a, T> {
    fn new(call: impl FnMut(&T) + 'a) -> Self {
        Self {
            call: Rc::new(RefCell::new(call)),
        }
    }

    #[inline]
    fn call(&self, val: &T) {
        self.call.borrow_mut()(val);
    }
}

impl<'a, T> Clone for Subscriber<'a, T> {
    fn clone(&self) -> Self {
        Subscriber {
            call: Rc::clone(&self.call)
        }
    }
}

impl<'a, T> PartialEq<Self> for Subscriber<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.call, &other.call)
    }
}
