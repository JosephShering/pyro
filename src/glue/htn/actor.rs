use godot::prelude::*;
use nanoid::nanoid;
use statig::prelude::*;

use std::collections::VecDeque;

use super::{
    action_library::{ActionLibrary, ActionStatus},
    htn::HTN,
    htn_action::HTNAction,
    npc::NPCBlackboards,
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
        self.fsm.as_mut().unwrap().handle(&ActorEvent::Plan);
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
            ActorEvent::Plan => Transition(State::plan()),
            _ => Handled,
        }
    }

    #[state(
        entry_action = "start_next_action",
        exit_action = "stop_current_action"
    )]
    fn executing(&mut self, event: &ActorEvent) -> Outcome<State> {
        match event {
            ActorEvent::Tick { delta } => match self.tick_current_action(*delta) {
                Some(ActionStatus::OnGoing) => Handled,
                Some(ActionStatus::Success) if !self.plan.is_empty() => {
                    Transition(State::executing())
                }
                Some(ActionStatus::Failed) => Transition(State::plan()),
                _ => Transition(State::idle()),
            },

            ActorEvent::Plan => Transition(State::plan()),
        }
    }

    #[action]
    fn start_next_action(&mut self) {
        if self.plan.is_empty() {
            return;
        }

        let Some(action_name) = self.plan.pop_front() else {
            return;
        };

        let mut library = self.action_library.bind_mut();
        let Some(action_entry) = library.get_mut(&action_name) else {
            godot_warn!("No action called {action_name} found");
            return;
        };

        let Some(()) =
            NPCBlackboards::singleton()
                .bind_mut()
                .with_blackboard_mut(&self.id, |blackboard| {
                    action_entry.bind_mut().enter(blackboard.clone());
                    Some(());
                })
        else {
            let key = &self.id;
            godot_warn!("No blackboard found for key {key}");
            return;
        };
    }

    #[action]
    fn stop_current_action(&mut self) {
        let Some(current_action) = self.current_action.as_mut() else {
            return;
        };
        NPCBlackboards::singleton()
            .bind_mut()
            .with_blackboard_mut(&self.id, |blackboard| {
                current_action.bind_mut().exit(blackboard.clone());
                Some(())
            });
        self.current_action = None;
    }

    fn tick_current_action(&mut self, delta: f32) -> Option<ActionStatus> {
        let current_action = self.current_action.as_mut()?;
        NPCBlackboards::singleton()
            .bind_mut()
            .with_blackboard_mut(&self.id, |blackboard| {
                Some(current_action.bind_mut().update(blackboard.clone(), delta))
            })?
    }

    #[state]
    fn plan(&mut self) -> Outcome<State> {
        match self.htn.bind().plan(&self.id) {
            Some(plan) => {
                let is_eq = plan.iter().eq(self.original_plan.iter());

                if is_eq {
                    Transition(State::idle())
                } else {
                    self.original_plan = plan.clone().into();
                    self.plan = plan;
                    Transition(State::executing())
                }
            }
            None => Transition(State::idle()),
        }
    }
}
