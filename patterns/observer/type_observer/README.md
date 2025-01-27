# Observer pattern
Observer is a behavioral design pattern that lets you define a subscription mechanism to notify
multiple objects about any events that happen to the object they’re observing.

## Client code:
```rust
fn main() {
    let text = RefCell::new(String::new());
    let observer = Observer::new();

    let (red_listener, _, _, _) = create_events(&text, &observer);

    red_listener.deactivate();
    observer.send(RedEvent {});
    observer.send(FirstEvent { val: 555 });

    assert_eq!("first(555) second third", text.borrow().as_str());
}
```

## Code Explanation

### 1.
```rust
let text = RefCell::new(String::new());
let observer = Observer::new();
```
All changes will be applied to the text variable, which will go through all the events and be modified accordingly.
An observer is also created, serving as the central example in this code.

## 2.
```rust 
let (red_listener, _, _, _) = create_events(&text, &observer);
```
Here, four listeners are created, which will be triggered upon sending specific events.
We retrieve only the first listener because we will use it later, while the others are simply ignored.
The function body looks as follows:
```rust
fn create_events(
    text: &RefCell<String>,
    observer: &Observer,
) -> (Listener, Listener, Listener, Listener) {
    (
        observer.listen::<RedEvent>(|_, _| {
            text.borrow_mut().push_str("red event");
        }),
        observer.listen::<FirstEvent>(|first_event, events| {
            events.send(SecondEvent {
                message: format!("first({}), ", first_event.val),
            });
        }),
        observer.listen::<SecondEvent>(|second_event, events| {
            events.send(ThirdEvent {
                string_val: format!("{} second, ", second_event.message),
            });
        }),
        observer.listen::<ThirdEvent>(|third_event, _| {
            text.borrow_mut()
                .push_str(&format!("{} third, ", third_event.string_val));
        }),
    )
}
```
To subscribe to an event, you need to call:
`observer.listen::<EventName>(|Event, EventPool| { function body }).`

As shown in the create_events function, it creates four listeners. The `FirstEvent` triggers the `SecondEvent`,
which in 
turn triggers the `ThirdEvent`. Finally, the ThirdEvent modifies the text variable.
### 3.
```rust
red_listener.deactivate();
```
This demonstrates how to deactivate an active listener.
Now, this event will no longer trigger even if a message of its type is sent to the observer.
```rust
observer.send(RedEvent {});
```
This event will not trigger because the listener has been deactivated.

### 4.
```rust
observer.send(FirstEvent { val: 555 });
```
Finally, this code sends the `FirstEvent` with the parameter val = 555.
This event triggers a chain reaction, sending SecondEvent and then `ThirdEvent`, which ultimately modifies the 
text variable.
As a result, the text variable contains the string: `first(555) second third`, as confirmed by the test.
