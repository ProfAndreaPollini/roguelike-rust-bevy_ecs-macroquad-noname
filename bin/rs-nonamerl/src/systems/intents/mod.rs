mod drink;
mod move_entity;
mod pick;

use bevy_ecs::prelude::Entity;
use bevy_ecs::world::World;
use rs_nonamerl_core::prelude::GameMap;

use crate::components::Position;
use crate::tiles::TestTile;

pub use self::drink::*;
pub use self::move_entity::*;
pub use self::pick::*;

pub fn remove_item_from_cell(world: &mut World, position: &Position, item: Entity) {
    let game_map = world.get_resource_mut::<GameMap<TestTile>>().unwrap();
    let mut tile = game_map.get(position.x, position.y).unwrap();
    tile.items.retain(|tile_item| *tile_item != item);
    game_map.set(position.x, position.y, tile);
}
