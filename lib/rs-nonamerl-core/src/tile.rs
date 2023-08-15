use std::fmt::Debug;

use bevy_ecs::prelude::Entity;
use macroquad::{prelude::Color, texture::Texture2D};

#[derive(Debug, Clone)]
pub enum TileSpriteInfo {
    None,
    SpriteSheet(&'static str),
    SingleSprite(Texture2D),
    Fill(Color),
}

impl Default for TileSpriteInfo {
    fn default() -> Self {
        Self::None
    }
}

pub trait Tile:
    'static + Debug + Clone + Visible + Visited + FovOccluder + Walkable + ItemContainer
{
    fn sprite_info(&self) -> TileSpriteInfo {
        TileSpriteInfo::None
    }
}

pub trait Visible {
    fn is_visible(&self) -> bool {
        true
    }
    fn set_visible(&mut self, visible: bool) {}
}

pub trait Visited {
    fn is_visited(&self) -> bool {
        false
    }
    fn set_visited(&mut self, visited: bool) {}
}

pub trait Walkable {
    fn is_walkable(&self) -> bool {
        true
    }
}

pub trait ItemContainer {
    fn items(&self) -> Option<Vec<Entity>> {
        None
    }
    fn add_item(&mut self, _item: Entity) {}
    fn remove_item(&mut self, _item: Entity) {}
}

pub trait FovOccluder {
    const BLOCKED: VisibilityOcclusion = VisibilityOcclusion(0.);
    const VISIBLE: VisibilityOcclusion = VisibilityOcclusion(1.);
    fn block_visibility(&self) -> VisibilityOcclusion {
        Self::VISIBLE
    }
}

#[repr(transparent)]
#[derive(PartialEq)]
pub struct VisibilityOcclusion(f32);

impl VisibilityOcclusion {
    pub fn new(v: f32) -> Option<Self> {
        if (0.0..=1.0).contains(&v) {
            Some(Self(v))
        } else {
            None
        }
    }

    pub unsafe fn new_unchecked(v: f32) -> Self {
        Self(v)
    }
}

impl From<VisibilityOcclusion> for f32 {
    fn from(v: VisibilityOcclusion) -> Self {
        v.0
    }
}
