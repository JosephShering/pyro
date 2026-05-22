use godot::prelude::*;

macro_rules! get_component {
    ($node:expr, $t:ty) => {{
        $node
            .get_children()
            .iter_shared()
            .find_map(|child| child.try_cast::<$t>().ok())
    }};
}

pub(crate) use get_component;

macro_rules! has_component {
    ($node:expr, $t:ty) => {
        get_component!($node, $t).is_some()
    };
}

pub(crate) use has_component;
