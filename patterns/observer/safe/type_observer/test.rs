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
