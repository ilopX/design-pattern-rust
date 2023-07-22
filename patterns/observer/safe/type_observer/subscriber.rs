use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

type SubscriberCall<'a> = Rc<RefCell<dyn FnMut(&dyn Any) + 'a>>;

pub trait Event: Any + Sized {
    fn cast(a: &dyn Any) -> &Self {
        a.downcast_ref::<Self>().unwrap()
    }
}

pub struct Subscriber<'a> {
    pub event_id: TypeId,
    fun: SubscriberCall<'a>,
}

impl<'a> Subscriber<'a> {
    pub fn new<E: Event>(call: impl FnMut(&E) + 'a) -> Self {
        Self {
            event_id: TypeId::of::<E>(),
            fun: Self::convert_to_any_args(call),
        }
    }

    #[inline]
    pub fn call<E: Event>(&self, event: &E) {
        (RefCell::borrow_mut(&self.fun))(event);
    }

    fn convert_to_any_args<E: Event>(call: impl FnMut(&E)) -> SubscriberCall<'a> {
        let call = Rc::new(RefCell::new(call));
        unsafe { mem::transmute::<Rc<RefCell<dyn FnMut(&E)>>, SubscriberCall<'a>>(call) }
    }
}

impl<'a> Clone for Subscriber<'a> {
    fn clone(&self) -> Self {
        Subscriber {
            event_id: self.event_id.clone(),
            fun: Rc::clone(&self.fun),
        }
    }
}

impl<'a> PartialEq for Subscriber<'a> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.fun, &other.fun)
    }
}
