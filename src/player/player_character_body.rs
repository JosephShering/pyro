use godot::classes::input::MouseMode;
use godot::classes::{
    CapsuleShape3D, CharacterBody3D, CollisionShape3D, ICharacterBody3D, Input, InputEvent,
    InputEventMouseMotion, RayCast3D,
};
use godot::global::is_zero_approx;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base = CharacterBody3D)]
pub struct PlayerCharacterBody {
    #[export]
    move_speed: f64,

    #[export]
    acceleration: f64,

    #[export]
    friction: f64,

    #[export]
    air_acceleration: f64,

    #[export]
    air_friction: f64,

    #[export]
    jump_height: f64,

    #[export]
    time_to_peak: f64,

    #[export]
    time_to_ground: f64,

    #[export]
    step_height: f64,

    #[export]
    max_floor_angle: f64,

    #[export]
    min_look_angle: f32,

    #[export]
    max_look_angle: f32,

    #[export_group(name = "Node Dependencies")]
    #[export]
    collision_shape: Option<Gd<CollisionShape3D>>,

    #[export]
    gimbal: Option<Gd<Node3D>>,

    #[export]
    camera: Option<Gd<Node3D>>,

    #[export]
    below_raycast: Option<Gd<RayCast3D>>,

    #[export]
    ahead_raycast: Option<Gd<RayCast3D>>,

    input_direction: Vector2,
    world_input_direction: Vector2,
    wish_velocity: Vector2,
    last_is_on_floor: bool,
    world_jump_height: f64,
    mouse_movement: Vector2,

    base: Base<CharacterBody3D>,
}

#[godot_api]
impl ICharacterBody3D for PlayerCharacterBody {
    fn init(base: Base<CharacterBody3D>) -> Self {
        Self {
            move_speed: 4.0,
            acceleration: 5.0,
            friction: 5.0,
            air_acceleration: 1.0,
            air_friction: 0.25,
            jump_height: 1.1,
            time_to_peak: 0.5,
            time_to_ground: 0.42,
            step_height: 0.25,
            max_floor_angle: 45.0,
            min_look_angle: -89.0,
            max_look_angle: 89.0,

            collision_shape: None,
            gimbal: None,
            camera: None,
            below_raycast: None,
            ahead_raycast: None,

            input_direction: Vector2::ZERO,
            world_input_direction: Vector2::ZERO,
            wish_velocity: Vector2::ZERO,
            last_is_on_floor: false,
            world_jump_height: 0.0,
            mouse_movement: Vector2::ZERO,

            base,
        }
    }

    fn ready(&mut self) {
        self.last_is_on_floor = self.base().is_on_floor();
        self.world_jump_height = self.base().get_global_position().y as f64;

        Input::singleton().set_mouse_mode(MouseMode::CAPTURED);
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if let Ok(mouse_motion) = event.try_cast::<InputEventMouseMotion>() {
            let modifier = Vector2::new(0.015, 0.011);
            self.mouse_movement = mouse_motion.get_relative() * modifier;
        }
    }

    fn physics_process(&mut self, delta: f64) {
        self.update_input_direction();
        self.calculate_wish_velocity();
        self.fall(delta);
        self.update_velocity(delta);
        self.rotate_camera();
        self.handle_jump();

        self.base_mut().move_and_slide();
    }
}

#[godot_api]
impl PlayerCharacterBody {
    #[signal]
    fn on_land();

    #[signal]
    fn on_jump();

    fn time_to(&self) -> f64 {
        if self.base().get_velocity().y > 0.0 {
            self.time_to_peak
        } else {
            self.time_to_ground
        }
    }

    fn handle_jump(&mut self) {
        if Input::singleton().is_action_just_pressed("jump") && self.base().is_on_floor() {
            self.jump();
        }
    }

    #[func]
    pub fn jump(&mut self) {
        let mut velocity = self.base().get_velocity();
        velocity.y = (2.0 * self.jump_height / self.time_to_peak) as f32;
        self.base_mut().set_velocity(velocity);
    }

    fn update_input_direction(&mut self) {
        let input = Input::singleton();
        self.input_direction =
            input.get_vector("move_left", "move_right", "move_forward", "move_backward");

        let world_3d = self.base().get_global_basis()
            * Vector3::new(self.input_direction.x, 0.0, self.input_direction.y);
        self.world_input_direction = Vector2::new(world_3d.x, world_3d.z);
    }

    fn calculate_wish_velocity(&mut self) {
        self.wish_velocity = (self.move_speed as f32) * self.world_input_direction;
    }

    fn fall(&mut self, delta: f64) {
        if self.base().is_on_floor() {
            return;
        }

        let time_to = self.time_to();
        let mut velocity = self.base().get_velocity();
        velocity.y -= (2.0 * self.jump_height / (time_to * time_to) * delta) as f32;
        self.base_mut().set_velocity(velocity);
    }

    fn update_velocity(&mut self, delta: f64) {
        let current_velocity_3d = self.base().get_velocity();
        let current_velocity = Vector2::new(current_velocity_3d.x, current_velocity_3d.z);

        let mut acceleration = if self.wish_velocity.length() == 0.0 {
            if self.base().is_on_floor() {
                self.friction
            } else {
                self.air_friction
            }
        } else {
            if self.base().is_on_floor() {
                self.acceleration
            } else {
                self.air_acceleration
            }
        };

        acceleration = acceleration / delta;

        let next_velocity =
            current_velocity.move_toward(self.wish_velocity, (delta * acceleration) as f32);

        let mut velocity = self.base().get_velocity();
        velocity.x = next_velocity.x;
        velocity.z = next_velocity.y;
        self.base_mut().set_velocity(velocity);
    }

    fn rotate_camera(&mut self) {
        let horizontal = self.mouse_movement.x;
        if !is_zero_approx(horizontal as f64) {
            self.base_mut().rotate_y(-horizontal);
        }

        let vertical = self.mouse_movement.y;
        if !is_zero_approx(vertical as f64) {
            if let Some(camera) = self.camera.as_mut() {
                camera.rotate_x(-vertical);
            }
        }

        if let Some(camera) = self.camera.as_mut() {
            let mut deg = camera.get_rotation_degrees();
            let x: f32 = deg.x;
            deg.x = x.clamp(self.min_look_angle, self.max_look_angle);
            camera.set_rotation_degrees(deg);
        }

        self.mouse_movement = Vector2::ZERO;
    }

    #[allow(dead_code)]
    fn setup_step_raycasts(&mut self) {
        let step_height = self.step_height as f32;
        let margin = 0.01_f32;

        let radius = self
            .collision_shape
            .as_ref()
            .and_then(|cs| cs.get_shape())
            .and_then(|s| s.try_cast::<CapsuleShape3D>().ok())
            .map(|c| c.get_radius())
            .unwrap_or(0.0);

        if let Some(below) = self.below_raycast.as_mut() {
            below.set_target_position(Vector3::new(0.0, -step_height, 0.0));
        }

        if let Some(ahead) = self.ahead_raycast.as_mut() {
            ahead.set_target_position(Vector3::new(0.0, -step_height, 0.0));
            ahead.set_position(Vector3::new(0.0, step_height + margin, -radius + margin));
        }
    }

    #[allow(dead_code)]
    fn is_surface_too_steep(&self, normal: Vector3) -> bool {
        normal.angle_to(Vector3::UP) > self.base().get_floor_max_angle()
    }
}
