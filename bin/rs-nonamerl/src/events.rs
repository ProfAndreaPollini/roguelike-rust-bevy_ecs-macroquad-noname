use bevy_ecs::prelude::Event;

use crate::{components::Position, resources::GameState};

#[derive(Debug, Event)]
pub struct ChangeGameStateEvent {
    pub new_state: GameState,
}

impl ChangeGameStateEvent {
    pub fn new(new_state: GameState) -> Self {
        Self { new_state }
    }
}

#[derive(Debug, Event)]
pub struct UpdateAvailableInteractionsEvent {
    pub position: Position,
}
