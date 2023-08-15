use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};

use bevy_ecs::{
    prelude::Entity,
    system::{Commands, Resource},
};
use macroquad::ui::Skin;
use rs_nonamerl_core::{prelude::KeyInput, IntVector2};

use crate::{
    components::{Interaction, MoveIntent},
    tiles::TestTile,
};

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
    current_tile: Option<TestTile>,
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

// type UseKindFn = fn(&mut Commands, Entity);

#[derive(Clone, Debug, Resource)]
pub struct UiConfig {
    pub skin: Skin,
    pub label_title_skin: Skin,
}

// #[derive(Clone)]
// pub struct UseKind<T> {
//     pub key: KeyInput,
//     pub op: UseKindFn,
//     pub _phantom: std::marker::PhantomData<T>,
// }

// impl<T> Debug for UseKind<T> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("UseKind").field("key", &self.key).finish()
//     }
// }

// impl UseKind<MoveIntent> {
//     pub fn new_move(key: KeyInput) -> Self {
//         Self {
//             key,
//             op: |commands, entity| {
//                 commands.entity(entity).insert(MoveIntent {
//                     target: IntVector2::new(0, 0),
//                 });
//             },
//             _phantom: std::marker::PhantomData,
//         }
//     }
// }

// pub trait ItemUseTrait {
//     fn use_item(&self, commands: &mut Commands, entity: Entity);
// }

// pub struct UseKindMoveIntent {}

// impl ItemUseTrait for UseKindMoveIntent {
//     fn use_item(&self, commands: &mut Commands, entity: Entity) {
//         commands.entity(entity).insert(MoveIntent {
//             target: IntVector2::new(0, 0),
//         });
//     }
// }

// // #[derive(Clone, Default, Resource)]
// // pub struct ItemUses {
// //     pub uses: Arc<Mutex<Vec<Box<&'static dyn ItemUseTrait>>>>,
// // }
