use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

type StateParent = Option<Weak<RefCell<State>>>;
type StateRef = Rc<RefCell<State>>;

type OnEnterFunc = Box<dyn FnMut()>;
type OnTickFunc = Box<dyn FnMut(f32)>;
type OnExitFunc = Box<dyn FnMut()>;

pub struct State {
    parent: StateParent,
    children: Vec<StateRef>,

    pub on_enter: Option<OnEnterFunc>,
    pub on_tick: Option<OnTickFunc>,
    pub on_exit: Option<OnExitFunc>,
}

impl State {
    pub fn new(
        on_enter: Option<OnEnterFunc>,
        on_tick: Option<OnTickFunc>,
        on_exit: Option<OnExitFunc>,
    ) -> Self {
        Self {
            parent: None,
            children: vec![],
            on_enter,
            on_tick,
            on_exit,
        }
    }

    pub fn enter(&mut self) {
        if let Some(on_enter) = &mut self.on_enter {
            on_enter();
        }
    }

    pub fn tick(&mut self, delta: f32) {
        if let Some(on_tick) = &mut self.on_tick {
            on_tick(delta);
        }
    }

    pub fn exit(&mut self) {
        if let Some(on_exit) = &mut self.on_exit {
            on_exit();
        }
    }

    pub fn add_child(self_rc: &StateRef, child: StateRef) {
        child.borrow_mut().parent = Some(Rc::downgrade(self_rc));
        self_rc.borrow_mut().children.push(child);
    }
}
