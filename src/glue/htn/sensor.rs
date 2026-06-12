use crate::glue::htn::npc::NPCBlackboards;

use super::actor::Actor;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Node)]
struct Sensor {
    #[export]
    actor: OnEditor<Gd<Actor>>,

    #[export]
    target: OnEditor<Gd<Node>>,

    #[export]
    key: StringName,

    base: Base<Node>,
}

#[godot_api]
impl INode for Sensor {
    fn physics_process(&mut self, _delta: f32) {
        let mut blackboards = NPCBlackboards::singleton();
        let key = &self.actor.bind().id;
        let value = self.target.get(&self.key);

        blackboards.set(key, &value);
    }
}
