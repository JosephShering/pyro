// use godot::{
//     classes::{RayCast3D, ShapeCast3D},
//     prelude::*,
// };

// use crate::{component::get_component, interaction::interactable::Interactable};

// #[derive(GodotClass)]
// #[class(init, base=Node)]
// struct Interactor {
//     #[export]
//     rays: Array<Gd<RayCast3D>>,

//     #[export]
//     shapes: Array<Gd<ShapeCast3D>>,

//     base: Base<Node>,
// }

// #[godot_api]
// impl INode for Interactor {}

// #[godot_api]
// impl Interactor {
//     #[func]
//     fn interact(&self) {
//         for ray in self.rays.iter_shared() {
//             if !ray.is_colliding() {
//                 continue;
//             }
//             if let Some(collider) = ray.get_collider() {
//                 let node: Gd<Node> = collider.cast::<Node>();
//                 if let Some(mut c) = get_component!(node, Interactable) {
//                     c.bind_mut().interact();
//                 }
//             }
//         }

//         for shape in self.shapes.iter_shared() {
//             if !shape.is_colliding() {
//                 continue;
//             }
//             for i in 0..shape.get_collision_count() {
//                 if let Some(collider) = shape.get_collider(i) {
//                     let node: Gd<Node> = collider.cast::<Node>();
//                     if let Some(mut c) = get_component!(node, Interactable) {
//                         c.bind_mut().interact();
//                         break; // only interact once per shape
//                     }
//                 }
//             }
//         }
//     }
// }
