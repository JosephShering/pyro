use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Node)]
struct Registry {
    items: Vec<Gd<Node3D>>,
    base: Base<Node>,
}

#[godot_api]
impl Registry {
    #[func]
    pub fn register(&mut self, item: Gd<Node3D>) {
        self.items.push(item);
    }

    #[func]
    pub fn unregister(&mut self, item: Gd<Node3D>) {
        self.items.retain(|c| *c != item);
    }

    #[func]
    pub fn closest(&mut self, point: Vector3) -> Option<Gd<Node3D>> {
        self.items
            .iter()
            .min_by(|c1, c2| {
                let pos1 = c1.get_global_position();
                let pos2 = c2.get_global_position();
                let dist1 = point.distance_squared_to(pos1);
                let dist2 = point.distance_squared_to(pos2);

                dist1.total_cmp(&dist2)
            })
            .cloned()
    }

    #[func]
    pub fn find(&mut self, node: Gd<Node3D>) -> Option<Gd<Node3D>> {
        self.items
            .iter()
            .find(|item| item.instance_id() == node.instance_id())
            .cloned()
    }

    #[func]
    pub fn filter(&mut self, f: Callable) -> Array<Gd<Node3D>> {
        self.items
            .iter()
            .filter(|item| f.call(vslice![**item]).booleanize())
            .cloned()
            .collect()
    }

    #[func]
    pub fn reduce(&mut self, default: Variant, f: Callable) -> Variant {
        self.items
            .iter()
            .fold(default, |acc, item| f.call(vslice![acc, *item]))
    }

    #[func]
    pub fn any(&mut self, f: Callable) -> bool {
        self.items
            .iter()
            .any(|item| f.call(vslice![*item]).booleanize())
    }

    #[func]
    pub fn all(&mut self, f: Callable) -> bool {
        self.items
            .iter()
            .all(|item| f.call(vslice![*item]).booleanize())
    }
}
