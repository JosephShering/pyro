use godot::prelude::*;

use crate::htn::{Plan, blackboard::Blackboard, operator::operator::Operator, selector::Selector};

#[derive(GodotClass)]
#[class(init, tool, base=Node)]
struct HtnPlanner {
    #[export]
    pub selector: OnEditor<Gd<Selector>>,

    #[export]
    pub blackboard: Option<Gd<Blackboard>>,

    #[var]
    pub operators: Array<Gd<Operator>>,

    #[export_tool_button(fn = Self::do_plan)]
    TestMyCode: PhantomVar<Callable>,

    base: Base<Node>,
}

#[godot_api]
impl INode for HtnPlanner {
    fn ready(&mut self) {
        if let Some(mut blackboard) = self.blackboard.clone() {
            blackboard
                .bind_mut()
                .signals()
                .on_changed()
                .connect_other(&*self, Self::on_blackboard_changed);
        }
    }
}

#[godot_api]
impl HtnPlanner {
    fn plan(&mut self) {
        if let Some(blackboard) = &self.blackboard {
            let data = blackboard.bind().data.duplicate_deep();
            let (operators, _new_hashmap) = self.selector.bind().decompose(data);

            self.operators = operators;
        }
    }

    fn on_blackboard_changed(&mut self) {
        self.plan();
    }

    #[func]
    fn do_plan(&mut self) {
        self.plan();

        let op = &self.operators;
        godot_print!("{op}");
    }
}
