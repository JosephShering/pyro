use crate::ai::htn::npc_blackboards::NPCBlackboards;

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
    keys: Array<StringName>,

    base: Base<Node>,
}

#[godot_api]
impl INode for Sensor {
    fn physics_process(&mut self, _delta: f32) {
        for key in self.keys.iter_shared() {
            let mut blackboards = NPCBlackboards::singleton();
            let blackboard_key = &self.actor.bind().id;
            let value = self.target.get(&key);

            blackboards
                .bind_mut()
                .with_blackboard_mut(blackboard_key, |blackboard| {
                    blackboard.bind_mut().set(key, value);
                    Some(())
                });
        }
    }
}
