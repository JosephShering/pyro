macro_rules! get_component {
    ($node:expr, $t:ty) => {{
        $node
            .get_children()
            .iter_shared()
            .find_map(|child| child.try_cast::<$t>().ok())
    }};
}

pub(crate) use get_component;
