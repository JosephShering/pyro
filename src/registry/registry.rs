use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Node)]
struct Registry {
    items: Vec<Gd<Node3D>>,
    /// Working set for the current query chain.
    staged: Vec<Gd<Node3D>>,
    /// Scalar result produced by a terminal-style op (reduce/any/all).
    value: Option<Variant>,
    /// Whether a chain is in progress (so `staged` is seeded lazily).
    started: bool,
    base: Base<Node>,
}

impl Registry {
    /// Seed the working set from `items` on the first link of a chain.
    fn begin(&mut self) {
        if !self.started {
            self.staged = self.items.clone();
            self.value = None;
            self.started = true;
        }
    }
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
    pub fn find(&mut self, node: Gd<Node3D>) -> Gd<Self> {
        self.begin();
        match self
            .staged
            .iter()
            .find(|item| item.instance_id() == node.instance_id())
            .cloned()
        {
            Some(found) => self.staged = vec![found],
            None => self.staged.clear(),
        }
        self.to_gd()
    }

    #[func]
    pub fn filter(&mut self, f: Callable) -> Gd<Self> {
        self.begin();
        self.staged
            .retain(|item| f.call(vslice![item.clone()]).booleanize());
        self.to_gd()
    }

    #[func]
    pub fn reduce(&mut self, default: Variant, f: Callable) -> Gd<Self> {
        self.begin();
        let result = self
            .staged
            .iter()
            .fold(default, |acc, item| f.call(vslice![acc, item.clone()]));
        self.value = Some(result);
        self.to_gd()
    }

    #[func]
    pub fn any(&mut self, f: Callable) -> Gd<Self> {
        self.begin();
        let result = self
            .staged
            .iter()
            .any(|item| f.call(vslice![item.clone()]).booleanize());
        self.value = Some(result.to_variant());
        self.to_gd()
    }

    #[func]
    pub fn all(&mut self, f: Callable) -> Gd<Self> {
        self.begin();
        let result = self
            .staged
            .iter()
            .all(|item| f.call(vslice![item.clone()]).booleanize());
        self.value = Some(result.to_variant());
        self.to_gd()
    }

    /// Keep only the items for which `f` returns false (inverse of `filter`).
    #[func]
    pub fn reject(&mut self, f: Callable) -> Gd<Self> {
        self.begin();
        self.staged
            .retain(|item| !f.call(vslice![item.clone()]).booleanize());
        self.to_gd()
    }

    /// Keep at most the first `count` items.
    #[func]
    pub fn take(&mut self, count: i64) -> Gd<Self> {
        self.begin();
        self.staged.truncate(count.max(0) as usize);
        self.to_gd()
    }

    /// Drop the first `count` items.
    #[func]
    pub fn skip(&mut self, count: i64) -> Gd<Self> {
        self.begin();
        let n = (count.max(0) as usize).min(self.staged.len());
        self.staged.drain(..n);
        self.to_gd()
    }

    /// Reverse the order of the staged items.
    #[func]
    pub fn reverse(&mut self) -> Gd<Self> {
        self.begin();
        self.staged.reverse();
        self.to_gd()
    }

    /// Remove duplicate items, keeping the first occurrence of each node.
    #[func]
    pub fn unique(&mut self) -> Gd<Self> {
        self.begin();
        let mut seen = std::collections::HashSet::new();
        self.staged.retain(|item| seen.insert(item.instance_id()));
        self.to_gd()
    }

    /// Sort the staged items with a custom comparator. `f(a, b)` should return
    /// true when `a` must come before `b` (same contract as `Array.sort_custom`).
    #[func]
    pub fn sort(&mut self, f: Callable) -> Gd<Self> {
        self.begin();
        self.staged.sort_by(|a, b| {
            if f.call(vslice![a.clone(), b.clone()]).booleanize() {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        });
        self.to_gd()
    }

    /// Map each item to a value via `f`; the chain now yields the mapped `Array`.
    #[func]
    pub fn map(&mut self, f: Callable) -> Gd<Self> {
        self.begin();
        let mapped: Array<Variant> = self
            .staged
            .iter()
            .map(|item| f.call(vslice![item.clone()]))
            .collect();
        self.value = Some(mapped.to_variant());
        self.to_gd()
    }

    /// Yield the number of staged items.
    #[func]
    pub fn count(&mut self) -> Gd<Self> {
        self.begin();
        self.value = Some((self.staged.len() as i64).to_variant());
        self.to_gd()
    }

    /// Yield the first staged item, or `null` if empty.
    #[func]
    pub fn first(&mut self) -> Gd<Self> {
        self.begin();
        self.value = Some(self.staged.first().cloned().to_variant());
        self.to_gd()
    }

    /// Yield the last staged item, or `null` if empty.
    #[func]
    pub fn last(&mut self) -> Gd<Self> {
        self.begin();
        self.value = Some(self.staged.last().cloned().to_variant());
        self.to_gd()
    }

    /// Yield the item with the smallest key, where `f(item)` returns the key.
    #[func]
    pub fn min_by(&mut self, f: Callable) -> Gd<Self> {
        self.begin();
        let best = self
            .staged
            .iter()
            .map(|item| (f.call(vslice![item.clone()]).try_to::<f64>().unwrap_or(0.0), item.clone()))
            .min_by(|a, b| a.0.total_cmp(&b.0))
            .map(|(_, item)| item);
        self.value = Some(best.to_variant());
        self.to_gd()
    }

    /// Yield the item with the largest key, where `f(item)` returns the key.
    #[func]
    pub fn max_by(&mut self, f: Callable) -> Gd<Self> {
        self.begin();
        let best = self
            .staged
            .iter()
            .map(|item| (f.call(vslice![item.clone()]).try_to::<f64>().unwrap_or(0.0), item.clone()))
            .max_by(|a, b| a.0.total_cmp(&b.0))
            .map(|(_, item)| item);
        self.value = Some(best.to_variant());
        self.to_gd()
    }

    /// Run `f` once per staged item for its side effects, leaving the set intact.
    #[func]
    pub fn for_each(&mut self, f: Callable) -> Gd<Self> {
        self.begin();
        for item in &self.staged {
            f.call(vslice![item.clone()]);
        }
        self.to_gd()
    }

    /// Materialize the chain: returns the value produced by a terminal op
    /// (reduce/any/all/map/count/first/last/min_by/max_by), or the staged nodes
    /// as an `Array` otherwise. Resets the chain.
    #[func]
    pub fn collect(&mut self) -> Variant {
        self.started = false;
        if let Some(value) = self.value.take() {
            self.staged.clear();
            return value;
        }
        let nodes: Array<Gd<Node3D>> = std::mem::take(&mut self.staged).into_iter().collect();
        nodes.to_variant()
    }
}
