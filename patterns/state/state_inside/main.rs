fn main() {
    let mut toggle = Toggle::new();
    assert_eq!(toggle.name(), "first");

    toggle.switch();
    assert_eq!(toggle.name(), "second");

    toggle.switch();
    assert_eq!(toggle.name(), "third");

    toggle.switch();
    assert_eq!(toggle.name(), "first");
}


struct Toggle<'a> {
    state: Option<Box<dyn State + 'a>>,
}

impl<'a> Toggle<'a> {
    fn new() -> Self {
        Self {
            state: Some(Box::new(FirstState {}))
        }
    }

    fn switch(&mut self) {
        let curr_state = self.state.take().unwrap();
        curr_state.next_state(self);
    }

    fn name(&self) -> String {
        self.state.as_ref().unwrap().name()
    }

    fn set_state(&mut self, new_state: impl State + 'a) {
        self.state.replace(Box::new(new_state));
    }
}

trait State {
    fn name(&self) -> String;
    fn next_state(&self, toggle: &mut Toggle);
}


struct FirstState;

impl State for FirstState {
    fn name(&self) -> String {
        "first".into()
    }

    fn next_state(&self, toggle: &mut Toggle) {
        toggle.set_state(SecondState {});
    }
}


struct SecondState;

impl State for SecondState {
    fn name(&self) -> String {
        "second".into()
    }

    fn next_state(&self, toggle: &mut Toggle) {
        toggle.set_state(ThirdState {});
    }
}


struct ThirdState;

impl State for ThirdState {
    fn name(&self) -> String {
        "third".into()
    }

    fn next_state(&self, toggle: &mut Toggle) {
        toggle.set_state(FirstState {});
    }
}
