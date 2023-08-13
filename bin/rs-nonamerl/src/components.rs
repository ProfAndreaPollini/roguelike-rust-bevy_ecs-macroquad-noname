#![allow(dead_code)]
use bevy_ecs::{
    prelude::{Component, Entity},
    system::Command,
    world::World,
};
use rs_nonamerl_core::IntVector2;

#[derive(Component, Default, Debug, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component, Default, Debug, Clone)]
pub struct Player {}

#[derive(Component, Default, Debug, Clone)]
pub struct Enemy {}

#[derive(Component, Default, Debug, Clone)]
pub struct SpriteDrawInfo {
    pub sprite_info: &'static str,
}

#[derive(Component, Default, Debug, Clone)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

#[derive(Component, Default, Debug, Clone)]
pub struct MoveIntent {
    pub target: IntVector2,
}

pub enum Effect {
    None,
}

#[derive(Debug, Clone)]
pub struct MoveAction {
    pub entity: Entity,
    pub source: IntVector2,
    pub target: IntVector2,
}

impl Command for MoveAction {
    fn apply(self, world: &mut World) {
        let mut position = world.get_mut::<Position>(self.entity).unwrap();
        position.x = self.target.x;
        position.y = self.target.y;
    }
}
