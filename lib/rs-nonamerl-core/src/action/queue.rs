use bevy_ecs::{
    system::Resource,
    world::{self, World},
};

use crate::prelude::{GameMap, Tile};

use super::{ActionActivator, ActionRequest, EntityAction};

#[derive(Debug, Clone, PartialEq)]
pub enum EntityActivatorFunctionResult {
    Ok,
    Alternate(EntityAction),
    Cancel,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EntityActivatorFunction<T> {
    f: ActionActivator<T>,
    params: T,
}

#[derive(Debug, Clone, PartialEq, Default, Resource)]
pub struct EntityActionQueue {
    activations: Vec<EntityAction>,
}

pub trait EntityQueue {
    fn add(&mut self, action: EntityAction);
    fn process_one<T: Tile>(&mut self, game_map: &GameMap<T>) -> Option<EntityAction>;
    fn clear(&mut self);
    fn is_empty(&self) -> bool;
    fn len(&self) -> usize;
    fn apply_actions<T: Tile>(&mut self, game_map: &GameMap<T>, world: &mut World);
}

impl EntityActionQueue {
    pub fn new() -> Self {
        Self {
            activations: Vec::new(),
        }
    }
}

impl EntityQueue for EntityActionQueue {
    fn add(&mut self, action: EntityAction) {
        self.activations.push(action);
    }

    fn process_one<T: Tile>(&mut self, game_map: &GameMap<T>) -> Option<EntityAction> {
        let mut ret: Option<EntityAction> = None;
        while let Some(action) = self.activations.pop() {
            let res = action.activate(game_map);
            match res {
                EntityActivatorFunctionResult::Ok => {
                    ret.replace(action);
                }
                EntityActivatorFunctionResult::Alternate(action) => {
                    self.add(action);
                    // self.activate()
                }
                EntityActivatorFunctionResult::Cancel => {}
            }
            if ret.is_some() {
                break;
            }
        }
        ret
    }

    fn clear(&mut self) {
        self.activations.clear();
    }

    fn is_empty(&self) -> bool {
        self.activations.is_empty()
    }

    fn len(&self) -> usize {
        self.activations.len()
    }

    fn apply_actions<T: Tile>(&mut self, game_map: &GameMap<T>, world: &mut World) {
        println!("apply_actions");
        while let Some(action) = self.process_one(game_map) {
            println!("action: {:?}", action);
        }
    }
}

// #[derive(Debug, Clone, PartialEq, Default, Resource)]
// pub struct EntityActionQueueNG {
//     requests: Vec<Box<ActionRequest>>,
// }
