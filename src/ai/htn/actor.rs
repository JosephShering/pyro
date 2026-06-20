use godot::prelude::*;
use nanoid::nanoid;
use statig::prelude::*;

use std::collections::VecDeque;

use super::{
    action_library::{ActionLibrary, ActionStatus},
    htn::HTN,
    htn_action::HTNAction,
    npc_blackboards::NPCBlackboards,
};

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct Actor {
    #[export]
    action_library: Option<Gd<ActionLibrary>>,

    #[export]
    htn: Option<Gd<HTN>>,

    #[export]
    agent: Option<Gd<Node>>,

    #[export]
    thoughts_per_second: f32,
    time: f32,

    pub id: String,

    fsm: Option<StateMachine<ActorStateMachine>>,
}

#[godot_api]
impl INode for Actor {
    fn ready(&mut self) {
        let htn = self.htn.as_mut().expect("Htn must be set in Actor").clone();
        let action_library = self
            .action_library
            .as_mut()
            .expect("Action Library must be set in Actor")
            .clone();

        let id = nanoid!();

        let mut blackboards = NPCBlackboards::singleton();
        blackboards.bind_mut().register(id.clone());
        self.id = id;

        let asm = ActorStateMachine {
            htn: htn,
            action_library: action_library,
            id: self.id.clone(),
            current_action: None,
            original_plan: Vec::new(),
            plan: VecDeque::new(),
        };

        self.fsm = Some(asm.state_machine());
    }

    fn exit_tree(&mut self) {
        let mut blackboards = NPCBlackboards::singleton();
        blackboards.bind_mut().cleanup(self.id.clone());
    }

    fn physics_process(&mut self, delta: f32) {
        self.time += delta;

        let timeout = 1.0 / self.thoughts_per_second;

        while self.time > timeout {
            self.time -= timeout;

            self.fsm.as_mut().unwrap().handle(&ActorEvent::Plan);
        }

        self.fsm
            .as_mut()
            .unwrap()
            .handle(&ActorEvent::Tick { delta });
    }
}

pub enum ActorEvent {
    Plan,
    Tick { delta: f32 },
}

struct ActorStateMachine {
    id: String,
    htn: Gd<HTN>,
    action_library: Gd<ActionLibrary>,
    original_plan: Vec<String>,
    plan: VecDeque<String>,
    current_action: Option<Gd<HTNAction>>,
}

// TODO check if the preconditions are true for the action to commence.

#[state_machine(initial = "State::idle()")]
impl ActorStateMachine {
    #[state]
    fn idle(&mut self, event: &ActorEvent) -> Outcome<State> {
        match event {
            ActorEvent::Plan => match self.plan() {
                None => Handled,
                Some(_plan) => {
                    self.enter_next_action();
                    Transition(State::executing())
                }
            },
            ActorEvent::Tick { delta: _ } => Handled,
        }
    }

    #[state]
    fn executing(&mut self, event: &ActorEvent) -> Outcome<State> {
        match event {
            ActorEvent::Tick { delta } => {
                let current_action = self.current_action.as_mut().expect("Was set earlier");

                let Some(action_status) = NPCBlackboards::singleton()
                    .bind_mut()
                    .with_blackboard_mut(&self.id, |blackboard| {
                        current_action.bind_mut().update(blackboard.clone(), *delta)
                    })
                else {
                    godot_error!("GDScript _update function didn't return anything somehow");
                    return Transition(State::idle());
                };

                match (action_status, self.plan.is_empty()) {
                    (ActionStatus::OnGoing, _) => Handled,
                    (ActionStatus::Success, true) => {
                        self.exit_current_action();
                        Transition(State::idle())
                    }

                    (ActionStatus::Success, false) => {
                        self.exit_current_action();
                        self.enter_next_action();
                        Handled
                    }
                    (ActionStatus::Failed, _) => {
                        self.exit_current_action();
                        Transition(State::idle())
                    }
                }
            }

            ActorEvent::Plan => match self.plan() {
                Some(_plan) => {
                    self.exit_current_action();
                    self.enter_next_action();
                    Handled
                }
                None => {
                    if self.plan.is_empty() {
                        Transition(State::idle())
                    } else {
                        Handled
                    }
                }
            },
        }
    }

    fn enter_next_action(&mut self) {
        let Some(action_name) = self.plan.pop_front() else {
            return;
        };

        let mut library = self.action_library.bind_mut();
        let Some(action_entry) = library.get(&action_name) else {
            godot_warn!("No action called {action_name} found");
            return;
        };

        self.current_action = Some(action_entry);

        let current_action = self.current_action.as_mut().unwrap();
        NPCBlackboards::singleton()
            .bind_mut()
            .with_blackboard_mut(&self.id, |blackboard| {
                current_action.bind_mut().enter(blackboard.clone())
            });
    }

    fn exit_current_action(&mut self) {
        let current_action = self
            .current_action
            .as_mut()
            .expect("Cannot call exit_current_plan with no plan");

        NPCBlackboards::singleton()
            .bind_mut()
            .with_blackboard_mut(&self.id, |blackboard| {
                current_action.bind_mut().exit(blackboard.clone())
            });

        self.current_action = None;
    }

    fn plan(&mut self) -> Option<&VecDeque<String>> {
        let plan = self.htn.bind().plan(&self.id)?;

        let is_eq = plan.iter().eq(self.original_plan.iter());

        if is_eq {
            None
        } else {
            self.original_plan = plan.clone().into();
            self.plan = plan.clone();

            godot_print!("{:?}", self.plan);

            Some(&self.plan)
        }
    }
}
