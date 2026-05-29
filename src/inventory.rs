use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Resource)]
struct Inventory {
    #[export]
    width: i32,

    #[export]
    height: i32,

    base: Base<Resource>,
}

#[godot_api]
impl IResource for Inventory {}

#[godot_api]
impl Inventory {}

#[derive(GodotClass)]
#[class(init, base=Resource)]
struct InventorySlot {
    #[export]
    amount: i32,

    #[export]
    item: Option<Gd<Item>>,

    base: Base<Resource>,
}

#[derive(GodotClass)]
#[class(init, base=Resource)]
struct Item {
    #[export]
    item_name: GString,

    base: Base<Resource>,
}
