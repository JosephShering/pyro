// use std::collections::HashMap;

// use super::action::Action;

// type ActionFactory = Box<dyn Fn() -> Box<dyn Action>>;

// #[derive(Default)]
// pub struct ActionsRepo {
//     factories: HashMap<&'static str, ActionFactory>,
// }

// impl ActionsRepo {
//     pub fn new() -> Self {
//         Self::default()
//     }

//     pub fn register<A: Action + Default + 'static>(&mut self, name: &'static str) {
//         self.factories
//             .insert(name, Box::new(|| Box::new(A::default())));
//     }

//     pub fn spawn(&self, name: &str) -> Option<Box<dyn Action>> {
//         self.factories.get(name).map(|f| f())
//     }

//     pub fn contains(&self, name: &str) -> bool {
//         self.factories.contains_key(name)
//     }
// }
