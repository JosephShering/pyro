use godot::{
    classes::{CharacterBody3D, ICharacterBody3D},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
pub struct EnemyCharacterBody3D {
    #[export]
    rotation_speed: f32,

    base: Base<CharacterBody3D>,
}

#[godot_api]
impl ICharacterBody3D for EnemyCharacterBody3D {
    fn init(base: Base<CharacterBody3D>) -> Self {
        Self {
            rotation_speed: 30.0,
            base,
        }
    }

    fn physics_process(&mut self, delta: f32) {
        self.follow_velocity(delta);
    }
}

#[godot_api]
impl EnemyCharacterBody3D {
    fn follow_velocity(&mut self, delta: f32) {
        let velocity = self.base().get_velocity();
        let horizontal = Vector2::new(velocity.x, velocity.z);

        if horizontal.length_squared() < 0.05 {
            return;
        }

        let target_yaw = velocity.x.atan2(-velocity.z);
        let mut rotation = self.base().get_rotation();

        let blend = 1.0 - (0.5_f32).powf(self.rotation_speed * delta);
        rotation.y = rotation.y.lerp_angle(target_yaw, blend);

        self.base_mut().set_rotation(rotation);
    }
}
