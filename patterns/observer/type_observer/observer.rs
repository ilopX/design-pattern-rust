use crate::event::Event;
use crate::even_pool::EventPool;
use crate::observer_map::ObserverMap;
use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

const DEFAULT_BUFFER_SIZE: usize = 10;

pub struct Observer {
    observer_map: Rc<RefCell<ObserverMap>>,
}

impl Observer {
    pub fn new() -> Self {
        Self {
            observer_map: Rc::new(RefCell::new(ObserverMap::new(DEFAULT_BUFFER_SIZE))),
        }
    }

    pub fn listen<T: Event>(&self, listener_fn: impl FnMut(&Box<T>, &mut EventPool)) -> Listener {
        let new_listener = Listener::new(listener_fn);
        new_listener.activate(self);
        new_listener
    }

    pub fn remove_listener(&self, listener: &Listener) {
        listener.deactivate();
    }

    pub fn send(&self, event: impl Event) {
        self.observer_map.borrow_mut().send(event);
    }
}

type DynEventFn = Box<dyn FnMut(&Box<dyn Event>, &mut EventPool)>;

struct ListenerData {
    event_type: TypeId,
    parent: Option<Rc<RefCell<ObserverMap>>>,
    fun: DynEventFn,
}

pub struct Listener {
    data: Rc<RefCell<ListenerData>>,
}

impl Listener {
    pub fn new<T: Event>(listener_fn: impl FnMut(&Box<T>, &mut EventPool)) -> Self {
        let rc = ListenerData {
            parent: None,
            event_type: TypeId::of::<T>(),
            fun: Self::convert_to_dyn_event_fn(listener_fn),
        };

        Self {
            data: Rc::new(RefCell::new(rc)),
        }
    }

    #[inline]
    pub fn notify(&self, arg: &Box<dyn Event>, event_pool: &mut EventPool) {
        (self.data.borrow_mut().fun)(arg, event_pool);
    }

    #[inline]
    pub fn activate(&self, observer_parent: &Observer) {
        self.deactivate();
        let future_parent = &observer_parent.observer_map;
        self.data.borrow_mut().parent = Some(Rc::clone(future_parent));
        future_parent.borrow_mut().add(self);
    }

    pub fn deactivate(&self) {
        let parent = self.data.borrow_mut().parent.take();

        if let Some(parent) = parent {
            parent.borrow_mut().remove(self);
        }
    }

    pub fn event_type(&self) -> TypeId {
        self.data.borrow().event_type.clone()
    }

    fn convert_to_dyn_event_fn<T: Event>(
        listener_fn: impl FnMut(&Box<T>, &mut EventPool),
    ) -> DynEventFn {
        let new_listener = Box::new(listener_fn);
        unsafe {
            mem::transmute::<Box<dyn FnMut(&Box<T>, &mut EventPool)>, DynEventFn>(new_listener)
        }
    }
}

impl Clone for Listener {
    fn clone(&self) -> Self {
        let is_second_clone_exists = Rc::strong_count(&self.data) == 2;

        if is_second_clone_exists {
            panic!("The system cannot have more than two Listener clones.");
        }

        Self {
            data: self.data.clone(),
        }
    }
}

impl PartialEq for Listener {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.data, &other.data)
    }
}
