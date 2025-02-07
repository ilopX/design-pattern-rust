#![allow(dead_code)]

mod even_pool;
mod observer_map;
mod observer;
mod vec_listeners;

use crate::observer::{Event, Listener, Observer};
use std::cell::RefCell;

fn main() {
    let (observer, text) = create_observer_and_out_text();
    observer.send(RedEvent {});
    observer.send(FirstEvent { val: 555 });
    assert_eq!("first(555) second third", text.borrow().as_str());
}

fn create_observer_and_out_text() -> (Observer, RefCell<String>) {
    let text = RefCell::new(String::new());
    let observer = Observer::new();

    let (red_listener, _, _, _) = create_listeners(&text, &observer);
    
    // demonstrate how unsubscribe listener
    red_listener.deactivate();

    (observer, text)
}

fn create_listeners(
    text: &RefCell<String>,
    observer: &Observer,
) -> (Listener, Listener, Listener, Listener) {
    (
        create_red_listener(&observer, &text),
        create_first_listener(&observer),
        create_second_listener(&observer),
        create_third_listener(&observer, &text),
    )
}

fn create_red_listener(observer: &Observer, text: &RefCell<String>) -> Listener {
    observer.listen::<RedEvent>(|_, _| {
        text.borrow_mut().push_str("red event");
    })
}

fn create_first_listener(observer: &Observer) -> Listener {
    observer.listen::<FirstEvent>(|first_event, events| {
        events.send(SecondEvent {
            message: format!("first({}), ", first_event.val),
        });
    })
}

fn create_second_listener(observer: &Observer) -> Listener {
    observer.listen::<SecondEvent>(|second_event, events| {
        events.send(ThirdEvent {
            string_val: format!("{} second, ", second_event.message),
        });
    })
}

fn create_third_listener(observer: &Observer, text: &RefCell<String>) -> Listener {
    observer.listen::<ThirdEvent>(|third_event, _| {
        text.borrow_mut()
            .push_str(&format!("{} third, ", third_event.string_val));
    })
}

// This code implements a structure called Observer, designed as a centralized system for message
// processing. Observer performs the following tasks:
//
// 1. Message Reception: The structure accepts messages represented as other structures (objects)
// that may contain data or logic.
//
// 2. Message Processing: During processing, the Observer dispatches signals to subscribers that are
// registered to receive specific types of messages.
//
// 3. Nested Message Support: The structure allows messages to be sent within other messages being
// processed. These nested messages are queued and processed after the current layer of messages is
// completed.
//
// In summary, Observer provides an event-driven subscription and processing system with nested
// message handling capabilities, making it a versatile tool for complex asynchronous workflows.

////////////////////////////////////////////////////////////////////////////////////////////////////
// main function implementation

struct FirstEvent {
    val: u32,
}

struct SecondEvent {
    message: String,
}

struct ThirdEvent {
    string_val: String,
}

struct RedEvent {}

impl Event for FirstEvent {}

impl Event for SecondEvent {}

impl Event for ThirdEvent {}

impl Event for RedEvent {}

/*
#[cfg(test)]
mod observer_tests {
    use crate::{Event, Observer};
    use std::cell::{Cell, RefCell};

    #[test]
    fn add_listener_and_send() {
        let bool_val = Cell::new(false);
        let observer = Observer::new();

        observer.listen::<bool>(|val, _| bool_val.set(**val));
        observer.send(true);

        assert_eq!(bool_val.get(), true);
    }

    #[test]
    fn listen_and_send_and_check_event_args() {
        let counter = RefCell::new(0);
        let observer = Observer::new();

        struct SomeEvent(u32);
        impl Event for SomeEvent {}

        observer.listen::<SomeEvent>(|first_event, _| {
            counter.replace(first_event.0);
        });
        observer.send(SomeEvent(123456));

        assert_eq!(*counter.borrow(), 123456);
    }

    #[test]
    fn listen_and_remove_listener_and_send() {
        let bool_val = Cell::new(false);
        let observer = Observer::new();

        let listener = observer.listen::<u32>(|_, _| {
            bool_val.replace(true);
        });

        listener.deactivate();
        observer.send(100);

        assert_eq!(bool_val.get(), false);
    }

    #[test]
    fn send_from_other_event() {
        let text = RefCell::new(String::from(""));
        let observer = Observer::new();

        observer.listen::<String>(|event, events| {
            text.borrow_mut().push_str(event);
            events.send(true);
        });

        observer.listen::<bool>(|event, events| {
            let mut result = text.borrow_mut();
            result.push_str(&event.to_string());
            result.push_str(", ");
            events.send(100);
        });

        observer.listen::<u32>(|event, _| {
            let mut result = text.borrow_mut();
            result.push_str(&event.to_string());
        });

        observer.send("Hello, ".to_string());

        assert_eq!(*text.borrow(), "Hello, true, 100");
    }

    #[test]
    fn looping_send() {
        let vec = RefCell::new(vec![]);
        let observer = Observer::new();

        struct Foo;
        impl Event for Foo {}

        struct Bar;
        impl Event for Bar {}

        observer.listen::<Foo>(|_, events| {
            vec.borrow_mut().push("foo");
            events.send(Bar);
        });

        observer.listen::<Bar>(|_, events| {
            let mut result = vec.borrow_mut();
            result.push("bar");

            if result.len() < 6 {
                events.send(Foo);
            }
        });

        observer.send(Foo);
        assert_eq!(
            *vec.borrow(),
            vec!["foo", "bar", "foo", "bar", "foo", "bar"]
        );
    }

    #[test]
    fn several_identical_events() {
        let counter = Cell::new(0);
        let observer = Observer::new();

        observer.listen::<bool>(|_, _| counter.set(counter.get() + 1));
        observer.listen::<bool>(|event, _| {
            if **event == false {
                counter.set(counter.get() + 1)
            }
        });
        observer.listen::<bool>(|_, _| counter.set(counter.get() + 1));

        observer.send(true);

        assert_eq!(counter.get(), 2);
    }

    impl Event for bool {}

    impl Event for u32 {}

    impl Event for String {}
}


#[cfg(test)]
mod listener_tests {
    use crate::{Event, EventPool, Listener, Observer, VecListeners};

    #[test]
    fn new_listener() {
        let mut text = String::new();

        let listener = Listener::new::<bool>(|event, _| {
            if **event {
                text.push_str("Its work");
            }
        });

        notify(listener, true);
        assert_eq!("Its work", text.as_str());
    }

    #[test]
    fn to_vec() {
        let mut is_done = false;

        let listener = Listener::new::<bool>(|_, _| {
            is_done = true;
        });

        let mut list = Vec::from_listener(listener, 10);
        let listener = list.pop().unwrap();
        notify(listener, true);

        assert_eq!(true, is_done);
    }

    #[test]
    fn deactivate() {
        let mut is_ok = true;
        let observer = Observer::new();
        let listener = observer.listen::<bool>(|_, _| is_ok = false);
        listener.deactivate();
        observer.send(true);
        assert_eq!(is_ok, true);
    }

    fn notify(listener: Listener, val: impl Event) {
        listener.notify(&value_to_event(val), &mut EventPool::new())
    }

    fn value_to_event(val: impl Event) -> Box<dyn Event> {
        Box::new(val) as Box<dyn Event>
    }
}
*/
