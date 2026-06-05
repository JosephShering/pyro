use std::{borrow::Cow, collections::HashMap};

use crate::htn::value::Value;

enum ComparisonOp {
    E,
    NE,
    LT,
    GT,
    GTE,
    LTE,
}

struct Condition {
    blackboard_key: String,
    op: ComparisonOp,
    value: Value,
}

enum ArithmeticOp {
    Eq,
    Add,
    Sub,
    Mult,
    Div,
}

struct Effect {
    blackboard_key: String,
    op: ArithmeticOp,
    value: Value,
}

impl Effect {
    fn new(key: impl Into<String>, op: ArithmeticOp, value: Value) -> Self {
        Self {
            blackboard_key: key.into(),
            op,
            value,
        }
    }
}

impl Condition {
    fn new(key: impl Into<String>, op: ComparisonOp, value: Value) -> Self {
        Self {
            blackboard_key: key.into(),
            op,
            value,
        }
    }
}

enum Task {
    Selector(Selector),
    Sequence(Sequence),
    Action(Action),
}

impl Task {
    fn selector(preconditions: Vec<Condition>, tasks: Vec<Task>) -> Task {
        Self::Selector(Selector {
            preconditions,
            tasks,
        })
    }

    fn sequence(preconditions: Vec<Condition>, tasks: Vec<Task>) -> Task {
        Self::Sequence(Sequence {
            preconditions,
            tasks,
        })
    }

    fn action(preconditions: Vec<Condition>, effects: Vec<Effect>, action: String) -> Task {
        Self::Action(Action {
            preconditions,
            action,
            effects,
        })
    }
}

fn is_met(preconditions: &Vec<Condition>, data: &HashMap<String, Value>) -> bool {
    for precondition in preconditions.iter() {
        let key = &precondition.blackboard_key;

        if let Some(left_value) = data.get(key) {
            let right_value = &precondition.value;
            let is_true = match precondition.op {
                ComparisonOp::E => left_value == right_value,
                ComparisonOp::NE => left_value != right_value,
                ComparisonOp::LT => left_value < right_value,
                ComparisonOp::GT => left_value > right_value,
                ComparisonOp::GTE => left_value >= right_value,
                ComparisonOp::LTE => left_value <= right_value,
            };

            if !is_true {
                return false;
            }
        }
    }

    return true;
}

fn get_preconditions(task: &Task) -> &Vec<Condition> {
    match task {
        Task::Selector(selector) => &selector.preconditions,
        Task::Sequence(s) => &s.preconditions,
        Task::Action(a) => &a.preconditions,
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
        Task::Selector(Selector {
            preconditions: _,
            tasks,
        }) => {
            for task in tasks.iter() {
                let preconditions = get_preconditions(&task);
                if is_met(preconditions, data) {
                    return decompose(task, data);
                }
            }

            None
        }
        Task::Sequence(Sequence {
            preconditions: _,
            tasks,
        }) => {
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
        Task::Action(Action {
            preconditions: _,
            action,
            effects,
        }) => {
            let mut new_data = data.clone();

            apply_effects(effects, &mut new_data);
            return Some((vec![action.clone()], new_data));
        }
    }
}

struct Selector {
    preconditions: Vec<Condition>,
    tasks: Vec<Task>,
}

struct Sequence {
    preconditions: Vec<Condition>,
    tasks: Vec<Task>,
}

struct Action {
    preconditions: Vec<Condition>,
    action: String,
    effects: Vec<Effect>,
}

macro_rules! cond {
    ($key:literal == $val:expr) => {
        Condition::new($key, ComparisonOp::E, Value::from($val))
    };
    ($key:literal != $val:expr) => {
        Condition::new($key, ComparisonOp::NE, Value::from($val))
    };
    ($key:literal <= $val:expr) => {
        Condition::new($key, ComparisonOp::LTE, Value::from($val))
    };
    ($key:literal >= $val:expr) => {
        Condition::new($key, ComparisonOp::GTE, Value::from($val))
    };
    ($key:literal <  $val:expr) => {
        Condition::new($key, ComparisonOp::LT, Value::from($val))
    };
    ($key:literal >  $val:expr) => {
        Condition::new($key, ComparisonOp::GT, Value::from($val))
    };
}

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
        Task::action(vec![], vec![], $name.to_string())
    };
    ($name:literal, when = [$($cond:expr),* $(,)?]) => {
        Task::action(vec![$($cond),*], vec![], $name.to_string())
    };
    ($name:literal, when = [$($cond:expr),* $(,)?], effects = [$($eff:expr),* $(,)?]) => {
        Task::action(vec![$($cond),*], vec![$($eff),*], $name.to_string())
    };
}

macro_rules! selector {
    (when = [$($cond:expr),* $(,)?], children = [$($child:expr),* $(,)?]) => {
        Task::selector(vec![$($cond),*], vec![$($child),*])
    };
}

macro_rules! sequence {
    (when = [$($cond:expr),* $(,)?], children = [$($child:expr),* $(,)?]) => {
        Task::sequence(vec![$($cond),*], vec![$($child),*])
    };
}

macro_rules! blackboard {
    ($($key:literal => $val:expr),* $(,)?) => {{
        let mut m: HashMap<String, Value> = HashMap::new();
        $(m.insert($key.to_string(), Value::from($val));)*
        m
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_htn() {
        let mut blackboard = blackboard! {
            "health" => 40,
            "has_health_item" => false
        };

        let root = selector!(
            when = [],
            children = [action!(
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
            when = [],
            children = [
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
                selector!(when = [], children = [action!("do_nothing")]),
                sequence!(when = [], children = []),
                sequence!(when = [], children = []),
                sequence!(when = [], children = [])
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
