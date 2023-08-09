use bevy_ecs::{
    system::Resource,
    world::{self, World},
};

use crate::prelude::{GameMap, Tile};

use super::{ActionActivator, EntityAction};

pub trait EntityActionActivatorChecker<T> {
    fn check(&self, params: &T) -> EntityActivatorFunctionResult;
}

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

impl EntityActionQueue {
    pub fn new() -> Self {
        Self {
            activations: Vec::new(),
        }
    }

    pub fn add(&mut self, action: EntityAction) {
        self.activations.push(action);
    }

    pub fn process_one<T: Tile>(&mut self, game_map: &GameMap<T>) -> Option<EntityAction> {
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

    pub fn clear(&mut self) {
        self.activations.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.activations.is_empty()
    }

    pub fn len(&self) -> usize {
        self.activations.len()
    }

    pub fn apply_actions<T: Tile>(&mut self, game_map: &GameMap<T>, world: &mut World) {
        println!("apply_actions");
        while let Some(action) = self.process_one(game_map) {
            println!("action: {:?}", action);
        }
    }
}
