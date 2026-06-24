use godot::prelude::*;
use nanoid::nanoid;
use statig::prelude::*;

use std::collections::VecDeque;

use crate::ai::htn::action_library::{ActionEnterStatus, ActionUpdateStatus};

use super::{
    action_library::ActionLibrary, htn::HTN, htn_action::HTNAction, npc_blackboards::NPCBlackboards,
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

#[derive(Debug)]
enum StateMachineError {
    NoPlanFormed,
    NoPlanSet,
    SamePlanFormed,
    NoCurrentAction { source: &'static str },
    FailedAction { name: String },
    NoKeyFound { key: String },
}

// TODO check if the preconditions are true for the action to commence.

#[state_machine(initial = "State::idle()")]
impl ActorStateMachine {
    #[state]
    fn idle(&mut self, event: &ActorEvent) -> Outcome<State> {
        match event {
            ActorEvent::Plan => match self.handle_plan_event() {
                Some(_) => Transition(State::executing()),
                None => Transition(State::idle()),
            },
            ActorEvent::Tick { delta: _ } => Handled,
        }
    }

    #[state]
    fn executing(&mut self, event: &ActorEvent) -> Outcome<State> {
        match event {
            ActorEvent::Tick { delta } => match self.handle_tick_event(*delta) {
                Some(transition) => transition,
                None => Transition(State::idle()),
            },
            ActorEvent::Plan => {
                let _ = self.handle_plan_event();
                Handled
            }
        }
    }

    fn handle_plan_event(&mut self) -> Option<()> {
        let plan = self.call_plan()?;

        self.plan = plan.clone();
        self.original_plan = self.plan.clone().into();

        let _ = self.call_exit();
        self.pop_to_next_action()?;
        self.call_enter()
    }

    fn handle_tick_event(&mut self, delta: f32) -> Option<Outcome<State>> {
        let action_status = self.call_update(delta)?;

        // godot_print!("{:?}", action_status);

        match action_status {
            ActionUpdateStatus::Success => {
                self.call_exit()?;
                self.pop_to_next_action()?;
                self.call_enter()?;

                Some(Handled)
            }
            ActionUpdateStatus::Failed => {
                self.call_exit()?;
                Some(Transition(State::idle()))
            }
            ActionUpdateStatus::OnGoing => Some(Handled),
        }
    }

    fn pop_to_next_action(&mut self) -> Option<()> {
        let Some(action_name) = self.plan.pop_front() else {
            return None;
        };
        let mut library = self.action_library.bind_mut();
        self.current_action = library.get(&action_name);

        Some(())
    }

    fn call_update(&mut self, delta: f32) -> Option<ActionUpdateStatus> {
        let Some(action) = self.current_action.as_mut() else {
            godot_error!("No current_action found in call_update");
            return None;
        };

        NPCBlackboards::singleton()
            .bind_mut()
            .with_blackboard_mut(&self.id, |blackboard| {
                action.bind_mut().update(blackboard.clone(), delta)
            })
    }

    fn call_enter(&mut self) -> Option<()> {
        let Some(action) = self.current_action.as_mut() else {
            godot_error!("No current_action found in call_enter");
            return None;
        };

        let Some(action_status) = NPCBlackboards::singleton()
            .bind_mut()
            .with_blackboard_mut(&self.id, |blackboard| {
                action.bind_mut().enter(blackboard.clone())
            })
        else {
            return None;
        };

        match action_status {
            ActionEnterStatus::Success => Some(()),
            ActionEnterStatus::Failed => None,
        }
    }

    fn call_exit(&mut self) -> Option<()> {
        let Some(action) = self.current_action.as_mut() else {
            return None;
        };

        NPCBlackboards::singleton()
            .bind_mut()
            .with_blackboard_mut(&self.id, |blackboard| {
                action.bind_mut().exit(blackboard.clone())
            })
    }

    fn call_plan(&self) -> Option<VecDeque<String>> {
        let plan = self.htn.bind().plan(&self.id)?;
        let is_eq = plan.iter().eq(self.original_plan.iter());
        if is_eq { None } else { Some(plan.clone()) }
    }
}
