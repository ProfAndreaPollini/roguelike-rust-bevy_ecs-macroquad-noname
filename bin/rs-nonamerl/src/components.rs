use bevy_ecs::prelude::Component;
use rs_nonamerl_core::prelude::TileSpriteInfo;

#[derive(Component, Default, Debug, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component, Default, Debug, Clone)]
pub struct Player {}

#[derive(Component, Default, Debug, Clone)]
pub struct SpriteDrawInfo {
    pub sprite_info: &'static str,
}

#[derive(Component, Default, Debug, Clone)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}
