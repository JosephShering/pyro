use godot::{
    classes::{CharacterBody3D, ICharacterBody3D},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
pub struct EnemyCharacterBody3D {
    #[export]
    rotation_speed: f32,

    #[export]
    jump_height: f32,

    #[export]
    time_to_peak: f32,

    #[export]
    time_to_ground: f32,

    base: Base<CharacterBody3D>,
}

#[godot_api]
impl ICharacterBody3D for EnemyCharacterBody3D {
    fn init(base: Base<CharacterBody3D>) -> Self {
        Self {
            rotation_speed: 30.0,
            jump_height: 1.1,
            time_to_peak: 0.5,
            time_to_ground: 0.45,
            base,
        }
    }

    fn physics_process(&mut self, delta: f32) {
        self.rotate_with_velocity(delta);

        if !self.base().is_on_floor() {
            self.fall(delta);
        }

        self.base_mut().move_and_slide();
    }
}

#[godot_api]
impl EnemyCharacterBody3D {
    fn rotate_with_velocity(&mut self, delta: f32) {
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

    #[func]
    fn jump(&mut self) {
        let mut velocity = self.base().get_velocity();

        velocity.y = 2.0 * self.jump_height / self.time_to_peak;

        self.base_mut().set_velocity(velocity);
    }

    #[func]
    fn fall(&mut self, delta: f32) {
        let mut velocity = self.base().get_velocity();

        let time_to = if velocity.y <= 0.0 {
            self.time_to_ground
        } else {
            self.time_to_peak
        };

        velocity.y -= (2.0 * self.jump_height / (time_to * time_to)) * delta;

        self.base_mut().set_velocity(velocity);
    }
}
