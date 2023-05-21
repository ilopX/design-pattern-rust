use std::cell::RefCell;

fn main() {
    let accumulator = RefCell::new(vec![]);
    let mut ob = Observer::<i32>::new();

    ob.subscribe(|val| accumulator.borrow_mut().push(val.clone()));

    ob.notify(1);
    ob.notify(2);

    assert_eq!(accumulator.borrow().as_slice(), &[1, 2]);
}

struct Observer<'a, T> {
    subs: Vec<Box<dyn FnMut(&T) + 'a>>,
}

impl<'a, T> Observer<'a, T> {
    fn new() -> Self {
        Self {
            subs: vec![],
        }
    }

    fn subscribe(&mut self, call: impl FnMut(&T) + 'a) {
        self.subs.push(Box::new(call));
    }

    fn notify(&mut self, new_value: T) {
        self.subs
            .iter_mut()
            .for_each(|call| call(&new_value));
    }
}
