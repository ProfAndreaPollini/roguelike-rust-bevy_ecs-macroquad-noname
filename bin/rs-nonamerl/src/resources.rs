use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};

use bevy_ecs::system::Resource;
use macroquad::ui::Skin;

use crate::{components::Interaction, tiles::TestTile};

#[derive(Clone, Debug, Resource, Default)]
pub struct GameContext {
    pub state: GameState,
}

#[derive(Clone, Debug, PartialEq)]
pub enum GameState {
    None,
    PlayGame,
    ShowInventory,
}

impl Default for GameState {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Clone, Debug, Resource, Default)]
pub struct CurrentCellInfo {
    interactions: Arc<Mutex<Vec<Interaction>>>,
    pub current_tile: Option<TestTile>,
}

impl CurrentCellInfo {
    pub fn new() -> Self {
        Self {
            interactions: Arc::new(Mutex::new(Vec::new())),
            current_tile: None,
        }
    }

    pub fn len(&self) -> usize {
        self.interactions.lock().unwrap().len()
    }

    pub fn add_all(&mut self, interactions: Vec<Interaction>) {
        self.interactions.lock().unwrap().extend(interactions);
    }

    pub fn add_interaction(&mut self, interaction: Interaction) {
        self.interactions.lock().unwrap().push(interaction);
    }

    pub fn clear(&mut self) {
        self.interactions.lock().unwrap().clear();
    }

    pub fn interactions(&self) -> Vec<Interaction> {
        self.interactions.lock().unwrap().clone()
    }

    pub fn current_tile(&self) -> Option<TestTile> {
        self.current_tile.clone()
    }

    pub fn set_current_tile(&mut self, tile: TestTile) {
        self.current_tile = Some(tile);
    }
}

#[derive(Clone, Debug, Resource)]
pub struct UiConfig {
    pub skin: Skin,
    pub label_title_skin: Skin,
}
