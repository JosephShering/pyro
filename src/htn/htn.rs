use std::{borrow::Cow, collections::HashMap};

use crate::htn::{
    condition::{ComparisonOp, Condition, eval},
    value::Value,
};

#[derive(Clone, Debug)]
pub enum ArithmeticOp {
    Eq,
    Add,
    Sub,
    Mult,
    Div,
}

#[derive(Clone, Debug)]
pub struct Effect {
    blackboard_key: String,
    op: ArithmeticOp,
    value: Value,
}

impl Effect {
    pub fn new(key: impl Into<String>, op: ArithmeticOp, value: Value) -> Self {
        Self {
            blackboard_key: key.into(),
            op,
            value,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Task {
    Selector {
        preconditions: Vec<Condition>,
        tasks: Vec<Task>,
    },
    Sequence {
        preconditions: Vec<Condition>,
        tasks: Vec<Task>,
    },
    Action {
        preconditions: Vec<Condition>,
        action: String,
        effects: Vec<Effect>,
    },
}

fn is_met(preconditions: &Vec<Condition>, data: &HashMap<String, Value>) -> bool {
    preconditions.iter().all(|c| eval(c, data))
}

fn get_preconditions(task: &Task) -> &Vec<Condition> {
    match task {
        Task::Selector { preconditions, .. } => &preconditions,
        Task::Sequence { preconditions, .. } => &preconditions,
        Task::Action { preconditions, .. } => &preconditions,
    }
}

fn apply_effects(effects: &[Effect], data: &mut HashMap<String, Value>) {
    for effect in effects {
        let key = &effect.blackboard_key;
        let new_value = match effect.op {
            ArithmeticOp::Eq => Some(effect.value.clone()),
            ArithmeticOp::Add => data.get(key).map(|x| x + &effect.value),
            ArithmeticOp::Sub => data.get(key).map(|x| x - &effect.value),
            ArithmeticOp::Mult => data.get(key).map(|x| x * &effect.value),
            ArithmeticOp::Div => data.get(key).map(|x| x / &effect.value),
        };

        if let Some(v) = new_value {
            data.insert(key.clone(), v);
        }
    }
}

pub fn plan(task: &Task, data: &HashMap<String, Value>) -> Option<Vec<String>> {
    match decompose(task, data) {
        Some((plan, _)) => Some(plan),
        None => None,
    }
}

fn decompose(
    task: &Task,
    data: &HashMap<String, Value>,
) -> Option<(Vec<String>, HashMap<String, Value>)> {
    match task {
        Task::Selector { tasks, .. } => {
            for task in tasks.iter() {
                let preconditions = get_preconditions(&task);
                if is_met(preconditions, data) {
                    return decompose(task, data);
                }
            }

            None
        }
        Task::Sequence { tasks, .. } => {
            let mut full_plan: Vec<String> = vec![];
            let mut state: Cow<HashMap<String, Value>> = Cow::Borrowed(data);

            for task in tasks.iter() {
                let preconditions = get_preconditions(&task);
                if !is_met(preconditions, &state) {
                    return None;
                }

                let (plan, new_data) = decompose(&task, &state)?;
                full_plan.extend(plan);
                state = Cow::Owned(new_data);
            }

            return Some((full_plan, state.into_owned()));
        }
        Task::Action {
            action, effects, ..
        } => {
            let mut new_data = data.clone();

            apply_effects(effects, &mut new_data);
            return Some((vec![action.clone()], new_data));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! cond {
        ($key:literal == $val:expr) => {
            Condition::compare($key, ComparisonOp::E, Value::from($val))
        };
        ($key:literal != $val:expr) => {
            Condition::compare($key, ComparisonOp::NE, Value::from($val))
        };
        ($key:literal <= $val:expr) => {
            Condition::compare($key, ComparisonOp::LTE, Value::from($val))
        };
        ($key:literal >= $val:expr) => {
            Condition::compare($key, ComparisonOp::GTE, Value::from($val))
        };
        ($key:literal <  $val:expr) => {
            Condition::compare($key, ComparisonOp::LT, Value::from($val))
        };
        ($key:literal >  $val:expr) => {
            Condition::compare($key, ComparisonOp::GT, Value::from($val))
        };
    }

    // macro_rules! all {
    //     ($($c:expr),+ $(,)?) => { Condition::All(vec![$($c),+]) };
    // }

    // macro_rules! any {
    //     ($($c:expr),+ $(,)?) => { Condition::Any(vec![$($c),+]) };
    // }

    // macro_rules! not {
    //     ($c:expr) => {
    //         Condition::Not(Box::new($c))
    //     };
    // }

    macro_rules! effect {
        ($key:literal = $val:expr) => {
            Effect::new($key, ArithmeticOp::Eq, Value::from($val))
        };
        ($key:literal + $val:expr) => {
            Effect::new($key, ArithmeticOp::Add, Value::from($val))
        };
        ($key:literal - $val:expr) => {
            Effect::new($key, ArithmeticOp::Sub, Value::from($val))
        };
        ($key:literal * $val:expr) => {
            Effect::new($key, ArithmeticOp::Mult, Value::from($val))
        };
        ($key:literal / $val:expr) => {
            Effect::new($key, ArithmeticOp::Div, Value::from($val))
        };
    }

    macro_rules! action {
        ($name:literal) => {
            Task::Action{preconditions: vec![], effects: vec![], action: $name.to_string()}
        };
        ($name:literal, when = [$($cond:expr),* $(,)?]) => {
            Task::Action{preconditions: vec![$($cond),*], effects: vec![], action: $name.to_string()}
        };
        ($name:literal, when = [$($cond:expr),* $(,)?], effects = [$($eff:expr),* $(,)?]) => {
            Task::Action{preconditions: vec![$($cond),*], effects: vec![$($eff),*], action: $name.to_string()}
        };
    }

    macro_rules! selector {
        (conditions = [$($cond:expr),* $(,)?], tasks = [$($child:expr),* $(,)?]) => {
            Task::Selector{preconditions: vec![$($cond),*], tasks: vec![$($child),*]}
        };
    }

    macro_rules! sequence {
        (conditions = [$($cond:expr),* $(,)?], tasks = [$($child:expr),* $(,)?]) => {
            Task::Selector{preconditions: vec![$($cond),*], tasks: vec![$($child),*]}
        };
    }

    macro_rules! blackboard {
        ($($key:literal => $val:expr),* $(,)?) => {{
            let mut m: HashMap<String, Value> = HashMap::new();
            $(m.insert($key.to_string(), Value::from($val));)*
            m
        }};
    }

    #[test]
    fn simple_htn() {
        let mut blackboard = blackboard! {
            "health" => 40,
            "has_health_item" => false
        };

        let root = selector!(
            conditions = [],
            tasks = [action!(
                "heal",
                when = [cond!("health" < 50), cond!("has_heal_item" == true),]
            ),]
        );

        let (plan, _) = decompose(&root, &mut blackboard).expect("should not return none");
        assert_eq!(plan, vec!["heal"]);
    }

    #[test]
    fn is_met_returns_true() {
        let conditions = vec![cond!("is_true" == true)];

        let mut blackboard = blackboard!(
            "is_true" => true
        );

        let result = is_met(&conditions, &mut blackboard);

        assert!(result);
    }

    #[test]
    fn is_met_returns_false() {
        let conditions = vec![cond!("is_true" == true)];

        let mut blackboard = blackboard!(
            "is_true" => false
        );

        let result = is_met(&conditions, &mut blackboard);

        assert!(!result);
    }

    #[test]
    fn sequence_succeeds() {
        let root = sequence!(
            conditions = [],
            tasks = [
                action!(
                    "put_clothes_in_machine",
                    when = [cond!("has_dirty_laundry" == true)],
                    effects = [effect!("clothes_in" = true)]
                ),
                action!(
                    "put_in_detergent",
                    when = [cond!("clothes_in" == true), cond!("has_detergent" == true)],
                    effects = [effect!("has_detergent" = false)]
                ),
                action!("turn_machine_on", when = [cond!("clothes_in" == true)]),
                selector!(conditions = [], tasks = [action!("do_nothing")]),
                sequence!(conditions = [], tasks = []),
                sequence!(conditions = [], tasks = []),
                sequence!(conditions = [], tasks = [])
            ]
        );

        let mut blackboard = blackboard! {
            "has_dirty_laundry" => true,
            "has_detergent" => true,
            "has_clean_laundry" => false,
            "clothes_in" => false
        };

        let (plan, _) = decompose(&root, &mut blackboard).expect("should not return none");
        assert_eq!(
            plan,
            vec![
                "put_clothes_in_machine",
                "put_in_detergent",
                "turn_machine_on",
                "do_nothing"
            ]
        )
    }
}
