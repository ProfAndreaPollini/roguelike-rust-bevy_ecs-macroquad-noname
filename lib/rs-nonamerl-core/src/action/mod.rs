mod queue;

use bevy_ecs::{prelude::Entity, world::World};
pub use queue::*;

use crate::{
    prelude::{GameMap, Tile},
    IntVector2,
};

#[derive(Debug, Clone, PartialEq)]
pub struct AttackActionParams {
    target: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MoveActionParams {
    pub dx: IntVector2,
    pub start: IntVector2,
    pub entity: Entity,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TakeDamageActionParams {
    pub target: Entity,
    pub damage: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AttackAction {
    activate: ActionActivator<AttackActionParams>,
}

pub type MoveActivationFn = fn(MoveActionParams) -> EntityActivatorFunctionResult;

#[derive(Debug, Clone, PartialEq)]
pub struct MoveAction {
    activate: ActionActivator<MoveActionParams>,
}

type ActionActivator<T> = fn(&T) -> bool;

#[derive(Debug, Clone, PartialEq)]
pub enum EntityAction {
    Move(MoveActionParams, Option<MoveActivationFn>),
    Attack(AttackActionParams),
    TakeDamage(TakeDamageActionParams),
    Wait,
    None,
}

impl Default for EntityAction {
    fn default() -> Self {
        EntityAction::None
    }
}

impl EntityAction {
    pub fn activate<T: Tile>(&self, game_map: &GameMap<T>) -> EntityActivatorFunctionResult {
        match self {
            EntityAction::Move(params, f) => {
                if let Some(f) = f {
                    return f(params.clone());
                }
                print!("move action activated: {:?}", params);

                let desired_position = params.start + params.dx;
                if let Some(tile) = game_map.get(desired_position.x, desired_position.y) {
                    if !tile.is_walkable() {
                        println!("tile blocked");
                        return EntityActivatorFunctionResult::Alternate(EntityAction::TakeDamage(
                            TakeDamageActionParams {
                                target: params.entity,
                                damage: 10,
                            },
                        ));
                    } else {
                        return EntityActivatorFunctionResult::Ok;
                    }
                }
                EntityActivatorFunctionResult::Ok
            }
            EntityAction::TakeDamage(params) => {
                println!("take damage action activated: {:?}", params);
                EntityActivatorFunctionResult::Ok
            }
            EntityAction::Attack(params) => {
                println!("attack action activated: {:?}", params);
                EntityActivatorFunctionResult::Ok
            }
            EntityAction::Wait => {
                println!("wait action activated");
                EntityActivatorFunctionResult::Ok
            }
            EntityAction::None => {
                println!("none action activated");
                EntityActivatorFunctionResult::Ok
            }
        }
    }
}
