use std::collections::VecDeque;

use godot::prelude::*;
use statig::{
    Outcome::{self, Handled, Transition},
    action, state, state_machine,
};

use crate::glue::{
    action_library::{Action, ActionLibrary},
    npc::NPCBlackboards,
};

#[derive(GodotConvert, Var, Export, Default, Clone)]
#[godot(via = GString)]
pub enum ActionStatus {
    #[default]
    Success,
    Failed,
    OnGoing,
}

enum ActorEvent {
    Tick(f32),
    SetActions(VecDeque<String>),
}

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct Actor {
    #[export]
    action_library: OnEditor<Gd<ActionLibrary>>,

    current_action: Option<Gd<Action>>,
    actor_id: String,
    plan: VecDeque<String>,

    #[export]
    thoughts_per_second: f32,
    time: f32,
}

#[state_machine(initial = "State::idle()")]
#[godot_api]
impl Actor {
    #[state]
    fn idle(&mut self, event: &ActorEvent) -> Outcome<State> {
        match event {
            ActorEvent::SetActions(new_plan) if !new_plan.is_empty() => {
                self.plan = new_plan.clone();
                Transition(State::executing())
            }
            _ => Handled,
        }
    }

    #[state(
        entry_action = "start_next_action",
        exit_action = "stop_current_action"
    )]
    fn executing(&mut self, event: &ActorEvent) -> Outcome<State> {
        match event {
            ActorEvent::Tick(delta) => match self.tick_current_action(*delta) {
                Some(ActionStatus::OnGoing) => Handled,
                Some(ActionStatus::Success) if !self.plan.is_empty() => {
                    // Self-transition: fires exit_current_action, then start_next_action
                    // — which advances the queue.
                    Transition(State::executing())
                }
                _ => Transition(State::idle()),
            },
            ActorEvent::SetActions(new_actions) => {
                // Replace the queue; the self-transition's exit will clean up the
                // currently-running action, then entry starts the new head.
                self.plan = new_actions.clone();
                if self.plan.is_empty() {
                    Transition(State::idle())
                } else {
                    Transition(State::executing())
                }
            }
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

        let Some(()) = NPCBlackboards::singleton()
            .bind()
            .with_blackboard(&self.actor_id, |data| {
                action_entry.bind().enter(data);
                Some(());
            })
        else {
            let key = &self.actor_id;
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
            .bind()
            .with_blackboard(&self.actor_id, |data| {
                current_action.bind_mut().exit(data.into());
                Some(())
            });
        self.current_action = None;
    }

    fn tick_current_action(&mut self, delta: f32) -> Option<ActionStatus> {
        let current_action = self.current_action.as_mut()?;
        NPCBlackboards::singleton()
            .bind()
            .with_blackboard(&self.actor_id, |data| {
                Some(current_action.tick(data, delta))
            })?
    }
}
