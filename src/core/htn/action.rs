use std::collections::HashMap;

use crate::core::Value;

pub enum ActionStatus {
    Success,
    Failed,
    OnGoing,
}

pub trait Action {
    fn enter(&mut self, data: &mut HashMap<String, Value>) {}
    fn tick(&mut self, data: &mut HashMap<String, Value>, delta: f32) -> ActionStatus {
        ActionStatus::Success
    }
    fn exit(&mut self, data: &mut HashMap<String, Value>) {}
}

// --- Action: three closures in a struct ---

type EnterFn = Box<dyn Fn(&mut HashMap<String, Value>) + Send + Sync>;
type TickFn = Box<dyn Fn(&mut HashMap<String, Value>, f32) -> ActionStatus + Send + Sync>;
type ExitFn = Box<dyn Fn(&mut HashMap<String, Value>) + Send + Sync>;

// pub struct Action {
//     enter: EnterFn,
//     tick: TickFn,
//     exit: ExitFn,
// }

// impl Action {
//     pub fn start() -> ActionBuilder {
//         ActionBuilder::default()
//     }

//     pub fn enter(&self, ctx: &mut HashMap<String, Value>) {
//         (self.enter)(ctx)
//     }

//     pub fn tick(&self, ctx: &mut HashMap<String, Value>, dt: f32) -> ActionStatus {
//         (self.tick)(ctx, dt)
//     }

//     pub fn exit(&self, ctx: &mut HashMap<String, Value>) {
//         (self.exit)(ctx)
//     }
// }

// // --- Builder ---

// #[derive(Default)]
// pub struct ActionBuilder {
//     enter: Option<EnterFn>,
//     tick: Option<TickFn>,
//     exit: Option<ExitFn>,
// }

// impl ActionBuilder {
//     pub fn enter<F>(mut self, f: F) -> Self
//     where
//         F: Fn(&mut HashMap<String, Value>) + Send + Sync + 'static,
//     {
//         self.enter = Some(Box::new(f));
//         self
//     }

//     pub fn tick<F>(mut self, f: F) -> Self
//     where
//         F: Fn(&mut HashMap<String, Value>, f32) -> ActionStatus + Send + Sync + 'static,
//     {
//         self.tick = Some(Box::new(f));
//         self
//     }

//     pub fn exit<F>(mut self, f: F) -> Self
//     where
//         F: Fn(&mut HashMap<String, Value>) + Send + Sync + 'static,
//     {
//         self.exit = Some(Box::new(f));
//         self
//     }

//     pub fn build(self) -> Action {
//         Action {
//             enter: self.enter.unwrap_or_else(|| Box::new(|_| {})),
//             tick: self
//                 .tick
//                 .unwrap_or_else(|| Box::new(|_, _| ActionStatus::Success)),
//             exit: self.exit.unwrap_or_else(|| Box::new(|_| {})),
//         }
//     }
// }
