use std::cell::RefCell;

use crate::observer::Observer;
use crate::subscriber::Event;

mod observer;
mod subscriber;

fn main() {
    let string_list = RefCell::new(String::new());
    let mut ob = Observer::new();

    ob.subscribe(|event: &FirstEvent| {
        let text = format!("name: {}, ", event.name);
        string_list.borrow_mut().push_str(&text);
    });

    let second_subscriber = ob.subscribe(|event: &SecondEvent| {
        let text = format!("num: {}", event.num);
        string_list.borrow_mut().push_str(&text);
    });

    ob.notify(FirstEvent { name: "ilopX" });
    ob.notify(SecondEvent { num: 100 });

    ob.unsubscribe(second_subscriber);
    ob.notify(SecondEvent { num: 200 }); // not be called

    assert_eq!(*string_list.borrow(), "name: ilopX, num: 100");
}

struct FirstEvent {
    name: &'static str,
}

impl Event for FirstEvent {}

struct SecondEvent {
    num: i32,
}

impl Event for SecondEvent {}
