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
        let key = &self.key;
        let blackboard_key = &self.actor.bind().id;
        let value = self.target.get(&self.key);

        {
            // godot_print!("{} {} {}", blackboard_key, key, value);
        }

        blackboards
            .bind_mut()
            .with_blackboard_mut(blackboard_key, |blackboard| {
                blackboard.bind_mut().set(key.into(), value);
                Some(())
            });
    }
}
