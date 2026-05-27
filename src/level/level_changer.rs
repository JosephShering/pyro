use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, singleton)]
struct LevelChanger {
    base: Base<Object>,
}

#[godot_api]
impl LevelChanger {
    #[func]
    pub fn change_level() {
        //Show loading screen
        //Start loading new scene async
        //QueueFree old scene
        //Wait for new scene to load
        //Add new scene to level root
    }
}
