use crate::even_pool::EventPool;
use crate::listener_map::ListenerMap;
use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

pub trait Event: Any {}

const DEFAULT_BUFFER_SIZE: usize = 10;

pub struct Observer {
    listener_map: Rc<RefCell<ListenerMap>>,
}

impl Observer {
    pub fn new() -> Self {
        Self {
            listener_map: Rc::new(RefCell::new(ListenerMap::new(DEFAULT_BUFFER_SIZE))),
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
        self.listener_map.borrow_mut().call(event);
    }
}

type DynEventFn = Box<dyn FnMut(&Box<dyn Event>, &mut EventPool)>;

struct ListenerRc {
    event_type: TypeId,
    parent: Option<Rc<RefCell<ListenerMap>>>,
    fun: DynEventFn,
}

pub struct Listener {
    rc: Rc<RefCell<ListenerRc>>,
}

impl Listener {
    pub fn new<T: Event>(listener_fn: impl FnMut(&Box<T>, &mut EventPool)) -> Self {
        let rc = ListenerRc {
            event_type: TypeId::of::<T>(),
            parent: None,
            fun: Self::convert_to_dyn_event_fn(listener_fn),
        };

        Self {
            rc: Rc::new(RefCell::new(rc)),
        }
    }

    #[inline]
    pub fn notify(&self, arg: &Box<dyn Event>, event_pool: &mut EventPool) {
        (self.rc.borrow_mut().fun)(arg, event_pool);
    }

    #[inline]
    pub fn activate(&self, future_parent: &Observer) {
        self.deactivate();
        let future_parent = &future_parent.listener_map;
        self.rc.borrow_mut().parent = Some(Rc::clone(future_parent));
        future_parent.borrow_mut().add(self);
    }

    pub fn deactivate(&self) {
        let parent = self.rc.borrow_mut().parent.take();

        if let Some(parent) = parent {
            parent.borrow_mut().remove(self);
        }
    }

    pub fn event_type(&self) -> TypeId {
        self.rc.borrow().event_type.clone()
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
        let is_second_clone_exists = Rc::strong_count(&self.rc) == 2;

        if is_second_clone_exists {
            panic!("");
        }

        Self {
            rc: self.rc.clone(),
        }
    }
}

impl PartialEq for Listener {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.rc, &other.rc)
    }
}
