use godot::prelude::*;
use itertools::Itertools;

#[derive(GodotClass)]
#[class(singleton, init, base=Node)]
struct Campfires {
    campfires: Vec<Gd<Campfire>>,
    base: Base<Node>,
}

#[godot_api]
impl INode for Campfires {}

#[godot_api]
impl Campfires {
    pub fn register(&mut self, campfire: Gd<Campfire>) {
        self.campfires.push(campfire);
    }

    pub fn unregister(&mut self, campfire: Gd<Campfire>) {
        self.campfires.retain(|c| *c != campfire);
    }

    #[func]
    pub fn closest_one(&mut self, point: Vector3) -> Option<Gd<Campfire>> {
        self.campfires
            .iter()
            .min_by(|c1, c2| {
                let pos1 = c1.get_position();
                let pos2 = c2.get_position();
                let dist1 = point.distance_squared_to(pos1);
                let dist2 = point.distance_squared_to(pos2);

                dist1.total_cmp(&dist2)
            })
            .cloned()
    }

    #[func]
    pub fn closest(&mut self, point: Vector3) -> Array<Gd<Campfire>> {
        let mut by_dist: Vec<(f32, Gd<Campfire>)> = self
            .campfires
            .iter()
            .map(|c| (point.distance_squared_to(c.get_position()), c.clone()))
            .collect();

        by_dist.sort_by(|(d1, _), (d2, _)| d1.total_cmp(d2));

        by_dist.into_iter().map(|(_, c)| c).collect()
    }
}

#[derive(GodotClass)]
#[class(init, base=Node3D)]
pub struct Campfire {
    base: Base<Node3D>,
}

#[godot_api]
impl INode3D for Campfire {
    fn ready(&mut self) {
        Campfires::singleton().bind_mut().register(self.to_gd());
    }

    fn exit_tree(&mut self) {
        Campfires::singleton().bind_mut().unregister(self.to_gd());
    }
}

#[godot_api]
impl Campfire {}

#[derive(GodotClass)]
#[class(init, base=Node3D)]
pub struct CampfireSeat {}
