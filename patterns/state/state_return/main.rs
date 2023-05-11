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


struct Toggle {
    state: Option<Box<dyn State>>,
}

impl Toggle {
    fn new() -> Self {
        Self {
            state: Some(Box::new(FirstState {}))
        }
    }

    fn switch(&mut self) {
        let curr_state = self.state.take().unwrap();
        let new_state = curr_state.next_state(self);
        self.state.replace(new_state);
    }

    fn name(&self) -> String {
        self.state.as_ref().unwrap().name()
    }
}


trait State {
    fn name(&self) -> String;
    fn next_state(&self, toggle_parent: &mut Toggle) -> Box<dyn State>;
}


struct FirstState;

impl State for FirstState {
    fn name(&self) -> String {
        "first".into()
    }

    fn next_state(&self, _toggle_parent: &mut Toggle) -> Box<dyn State> {
        Box::new(SecondState {})
    }
}


struct SecondState;

impl State for SecondState {
    fn name(&self) -> String {
        "second".into()
    }

    fn next_state(&self, _toggle_parent: &mut Toggle) -> Box<dyn State> {
        Box::new(ThirdState {})
    }
}


struct ThirdState;

impl State for ThirdState {
    fn name(&self) -> String {
        "third".into()
    }

    fn next_state(&self, _toggle_parent: &mut Toggle) -> Box<dyn State> {
        Box::new(FirstState {})
    }
}
